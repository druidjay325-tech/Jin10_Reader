use crate::{
    config::{McpConfig, normalize_endpoint, validate_token},
    error::{AppError, AppResult},
};
use rmcp::{
    ClientLifecycleMode, ClientServiceExt,
    model::{CallToolRequestParams, ClientCapabilities, ClientInfo, Implementation},
    transport::streamable_http_client::{
        StreamableHttpClientTransport, StreamableHttpClientTransportConfig,
    },
};
use serde_json::{Value, json};
use std::time::Duration;

pub type McpClient = rmcp::service::RunningService<rmcp::RoleClient, ClientInfo>;
// 预算分别覆盖初始化和一次业务调用；不把 SSE 长连接的存续期当作请求预算。
pub const CONNECT_BUDGET: Duration = Duration::from_secs(15);
pub const CALL_BUDGET: Duration = Duration::from_secs(20);

pub async fn bounded<T>(
    budget: Duration,
    operation: impl std::future::Future<Output = AppResult<T>>,
) -> AppResult<T> {
    tokio::time::timeout(budget, operation)
        .await
        .map_err(|_| AppError::timeout())?
}

pub async fn connect(cfg: &McpConfig) -> AppResult<McpClient> {
    let endpoint = normalize_endpoint(&cfg.server_url)?;
    validate_token(&cfg.bearer_token)?;
    if cfg.bearer_token.trim().is_empty() {
        return Err(AppError::unconfigured());
    }
    connect_transport(&endpoint, &cfg.bearer_token, CONNECT_BUDGET).await
}

async fn connect_transport(endpoint: &str, token: &str, budget: Duration) -> AppResult<McpClient> {
    let http = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(8))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| AppError::network())?;
    let transport = StreamableHttpClientTransport::with_client(
        http,
        StreamableHttpClientTransportConfig::with_uri(endpoint.to_owned())
            .auth_header(token.to_owned()),
    );
    let info = ClientInfo::new(
        ClientCapabilities::default(),
        Implementation::new("jin10-reader", env!("CARGO_PKG_VERSION")),
    );
    bounded(budget, async {
        info.serve_with_lifecycle(transport, ClientLifecycleMode::Initialize)
            .await
            .map_err(|error| classify_error(&error, true))
    })
    .await
}

// 只将分类送往 UI 和日志，不将上游原文、HTTP 地址或请求头带回前端。
pub fn classify_error(error: &(dyn std::error::Error + 'static), transport: bool) -> AppError {
    let mut cursor = Some(error);
    let mut messages = String::new();
    while let Some(source) = cursor {
        if let Some(http) = source.downcast_ref::<reqwest::Error>() {
            if let Some(status) = http.status() {
                if status.as_u16() == 401 || status.as_u16() == 403 {
                    return AppError::auth();
                }
                if status.as_u16() == 429 {
                    return AppError::quota();
                }
                if status.is_server_error() {
                    return AppError::network();
                }
                if status.is_client_error() {
                    return AppError::new(
                        "upstream",
                        "数据源拒绝了请求，请检查服务地址与参数",
                        false,
                    );
                }
            }
            if http.is_timeout() {
                return AppError::timeout();
            }
        }
        messages.push_str(&source.to_string().to_lowercase());
        messages.push(' ');
        cursor = source.source();
    }
    classify_message(&messages, transport)
}

pub fn classify_message(message: &str, transport: bool) -> AppError {
    let message = message.to_lowercase();
    if [
        "401",
        "403",
        "unauthorized",
        "forbidden",
        "auth required",
        "authentication failed",
        "invalid token",
        "invalid api key",
        "expired token",
        "token expired",
        "insufficient scope",
        "认证",
        "鉴权",
        "token无效",
        "token 无效",
    ]
    .iter()
    .any(|word| message.contains(word))
    {
        return AppError::auth();
    }
    if [
        "429",
        "quota",
        "rate limit",
        "too many requests",
        "配额",
        "限流",
        "次数",
        "额度",
        "限额",
        "今日已达上限",
        "达到调用上限",
        "请求过于频繁",
    ]
    .iter()
    .any(|word| message.contains(word))
    {
        return AppError::quota();
    }
    if message.contains("timeout") || message.contains("timed out") {
        return AppError::timeout();
    }
    if transport {
        AppError::network()
    } else {
        AppError::new(
            "upstream",
            "数据源未能完成该请求，请稍后重试或检查参数",
            false,
        )
    }
}

fn service_error(error: rmcp::ServiceError) -> AppError {
    match &error {
        rmcp::ServiceError::Timeout { .. } => AppError::timeout(),
        rmcp::ServiceError::TransportClosed | rmcp::ServiceError::TransportSend(_) => {
            classify_error(&error, true)
        }
        _ => classify_error(&error, false),
    }
}

pub fn parse_payload(text: &str) -> AppResult<Value> {
    if text.trim().is_empty() {
        return Err(AppError::new(
            "invalid_response",
            "数据源返回了空响应",
            false,
        ));
    }
    let value: Value = serde_json::from_str(text)
        .map_err(|_| AppError::new("invalid_response", "数据源返回了无法解析的响应", false))?;
    if let Some(status) = value.get("status") {
        let code = status
            .as_i64()
            .or_else(|| status.as_str().and_then(|s| s.parse().ok()));
        if let Some(code) = code
            && code != 200
            && code != 0
        {
            return Err(classify_message(
                &format!("{code} {}", value["message"].as_str().unwrap_or_default()),
                false,
            ));
        }
    }
    if value.get("success").and_then(Value::as_bool) == Some(false) {
        return Err(classify_message(
            value["message"].as_str().unwrap_or_default(),
            false,
        ));
    }
    Ok(value.get("data").cloned().unwrap_or(value))
}

async fn call(client: &McpClient, tool: &str, args: Value) -> AppResult<Value> {
    let params = CallToolRequestParams::new(tool.to_owned())
        .with_arguments(args.as_object().cloned().unwrap_or_default());
    let result = bounded(CALL_BUDGET, async {
        client.call_tool(params).await.map_err(service_error)
    })
    .await?;
    let texts = result
        .content
        .iter()
        .filter_map(|content| match content {
            rmcp::model::ContentBlock::Text(text) => Some(text.text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    if result.is_error == Some(true) {
        return Err(classify_message(&texts.join(" "), false));
    }
    if let Some(value) = result.structured_content {
        return parse_payload(&value.to_string());
    }
    parse_payload(&texts.join("\n"))
}

pub async fn list_tools(client: &McpClient) -> AppResult<Vec<String>> {
    let result = bounded(CALL_BUDGET, async {
        client
            .list_tools(Default::default())
            .await
            .map_err(service_error)
    })
    .await?;
    Ok(result
        .tools
        .iter()
        .map(|tool| tool.name.to_string())
        .collect())
}
pub async fn get_quote(client: &McpClient, code: &str) -> AppResult<Value> {
    call(client, "get_quote", json!({"code": code})).await
}
pub async fn get_quotes(client: &McpClient, codes: &[String]) -> AppResult<Vec<Value>> {
    let mut output = Vec::with_capacity(codes.len());
    for code in codes {
        output.push(get_quote(client, code).await?);
    }
    Ok(output)
}
pub async fn list_flash(client: &McpClient, cursor: Option<&str>) -> AppResult<Value> {
    let mut args = json!({});
    if let Some(cursor) = cursor {
        args["cursor"] = json!(cursor);
    }
    call(client, "list_flash", args).await
}
pub async fn list_news(client: &McpClient, cursor: Option<&str>) -> AppResult<Value> {
    let mut args = json!({});
    if let Some(cursor) = cursor {
        args["cursor"] = json!(cursor);
    }
    call(client, "list_news", args).await
}
pub async fn list_calendar(client: &McpClient) -> AppResult<Value> {
    call(client, "list_calendar", json!({})).await
}
pub async fn search_flash(
    client: &McpClient,
    keyword: &str,
    cursor: Option<&str>,
) -> AppResult<Value> {
    let mut args = json!({"keyword": keyword});
    if let Some(cursor) = cursor {
        args["cursor"] = json!(cursor);
    }
    call(client, "search_flash", args).await
}
pub async fn search_news(
    client: &McpClient,
    keyword: &str,
    cursor: Option<&str>,
    limit: Option<u32>,
) -> AppResult<Value> {
    let mut args = json!({"keyword": keyword});
    if let Some(cursor) = cursor {
        args["cursor"] = json!(cursor);
    }
    if let Some(limit) = limit {
        args["limit"] = json!(limit.to_string());
    }
    call(client, "search_news", args).await
}
pub async fn get_news(client: &McpClient, id: &str) -> AppResult<Value> {
    call(client, "get_news", json!({"id": id})).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    #[tokio::test]
    async fn stalled_handshake_times_out_without_external_network() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let hold = tokio::spawn(async move {
            let (_socket, _) = listener.accept().await.unwrap();
            std::future::pending::<()>().await;
        });
        let result = connect_transport(
            &format!("http://{address}/mcp"),
            "synthetic-test-value",
            Duration::from_millis(80),
        )
        .await;
        assert!(matches!(result, Err(error) if error.code == "timeout"));
        hold.abort();
    }
    #[tokio::test]
    async fn local_http_auth_and_quota_errors_are_not_retryable() {
        for (status, expected) in [(401, "auth_failed"), (429, "rate_limited")] {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let server = tokio::spawn(async move {
                loop {
                    let (mut socket, _) = listener.accept().await.unwrap();
                    let mut request = Vec::new();
                    let mut chunk = [0; 4096];
                    loop {
                        let read = socket.read(&mut chunk).await.unwrap();
                        if read == 0 {
                            break;
                        }
                        request.extend_from_slice(&chunk[..read]);
                        assert!(request.len() <= 65536, "假服务请求超出预期大小");
                        if let Some(end) = request.windows(4).position(|value| value == b"\r\n\r\n")
                        {
                            let headers = String::from_utf8_lossy(&request[..end]).to_lowercase();
                            let length = headers
                                .lines()
                                .find_map(|line| {
                                    line.strip_prefix("content-length:")
                                        .and_then(|value| value.trim().parse::<usize>().ok())
                                })
                                .unwrap_or(0);
                            if request.len() >= end + 4 + length {
                                break;
                            }
                        }
                    }
                    // 完整读取请求后再关闭连接，避免未读 POST 数据导致 TCP reset 掩盖 HTTP 状态。
                    let response = format!(
                        "HTTP/1.1 {status} Test\r\nContent-Type: text/plain\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                    );
                    socket.write_all(response.as_bytes()).await.unwrap();
                    socket.shutdown().await.unwrap();
                }
            });
            let result = connect_transport(
                &format!("http://{address}/mcp"),
                "fixture",
                Duration::from_secs(1),
            )
            .await;
            server.abort();
            assert!(
                matches!(&result, Err(error) if error.code == expected && !error.retryable),
                "本地 HTTP {status} 错误分类不符合预期: {:?}",
                result.as_ref().err()
            );
        }
    }
}

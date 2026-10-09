use jin10_reader::{config::McpConfig, mcp};
use serde_json::Value;

fn shape(value: &Value) -> &'static str {
    match value {
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        Value::Null => "null",
        _ => "scalar",
    }
}
fn first_id(value: &Value) -> Option<String> {
    let list = value.as_array().or_else(|| {
        ["list", "items", "news", "data"]
            .iter()
            .find_map(|key| value.get(key).and_then(Value::as_array))
    })?;
    list.iter().find_map(|item| match item.get("id") {
        Some(Value::String(id)) if !id.is_empty() => Some(id.clone()),
        Some(Value::Number(id)) => Some(id.to_string()),
        _ => None,
    })
}

#[tokio::test]
#[ignore = "需要显式在线授权与专用测试凭据，不读取本机配置"]
async fn online_reading_tools() {
    assert_eq!(
        std::env::var("JIN10_RUN_ONLINE").as_deref(),
        Ok("1"),
        "请通过显式在线测试入口运行"
    );
    let token = std::env::var("JIN10_TEST_TOKEN").expect("必须提供专用测试凭据 JIN10_TEST_TOKEN");
    assert!(!token.trim().is_empty(), "专用测试凭据不能为空");
    let config = McpConfig {
        bearer_token: token,
        ..Default::default()
    };
    let client = mcp::connect(&config).await.expect("初始化失败");
    let tools = mcp::list_tools(&client).await.expect("工具发现失败");
    for required in [
        "get_quote",
        "list_flash",
        "list_news",
        "list_calendar",
        "search_flash",
        "search_news",
        "get_news",
    ] {
        assert!(
            tools.iter().any(|name| name == required),
            "缺少产品必需工具 {required}"
        );
    }
    let quote = mcp::get_quote(&client, "XAUUSD").await.expect("行情失败");
    println!("1/7 get_quote: {}", shape(&quote));
    let flash = mcp::list_flash(&client, None).await.expect("快讯失败");
    println!("2/7 list_flash: {}", shape(&flash));
    let news = mcp::list_news(&client, None).await.expect("资讯失败");
    println!("3/7 list_news: {}", shape(&news));
    let calendar = mcp::list_calendar(&client).await.expect("日历失败");
    println!("4/7 list_calendar: {}", shape(&calendar));
    let flash = mcp::search_flash(&client, "黄金", None)
        .await
        .expect("快讯搜索失败");
    println!("5/7 search_flash: {}", shape(&flash));
    let search = mcp::search_news(&client, "黄金", None, Some(5))
        .await
        .expect("资讯搜索失败");
    println!("6/7 search_news: {}", shape(&search));
    let id = first_id(&news).expect("资讯没有可用 ID，详情测试未完成；本次在线验收失败");
    let detail = mcp::get_news(&client, &id).await.expect("资讯详情失败");
    println!("7/7 get_news: {}", shape(&detail));
    assert!(!detail.is_null(), "资讯详情不能是空响应");
    client.cancellation_token().cancel();
    println!("已验证 7 个保留工具；K 线不属于本次产品范围，未测试。");
}

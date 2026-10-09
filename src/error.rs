use serde::Serialize;
pub type AppResult<T> = Result<T, AppError>;
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}
impl AppError {
    pub fn new(code: &str, message: &str, retryable: bool) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            retryable,
        }
    }
    pub fn invalid(message: &str) -> Self {
        Self::new("invalid_input", message, false)
    }
    pub fn storage(message: &str) -> Self {
        Self::new("storage", message, false)
    }
    pub fn network() -> Self {
        Self::new("offline", "暂时无法连接数据源，请检查网络后重试", true)
    }
    pub fn timeout() -> Self {
        Self::new("timeout", "数据源响应超时，请稍后重试", true)
    }
    pub fn auth() -> Self {
        Self::new(
            "auth_failed",
            "认证失败，请在设置中检查凭据与服务权限",
            false,
        )
    }
    pub fn quota() -> Self {
        Self::new(
            "rate_limited",
            "数据源提示限流或配额不足，已暂停自动请求；请确认额度后手动重连",
            false,
        )
    }
    pub fn unconfigured() -> Self {
        Self::new("unconfigured", "请先在设置中连接数据源", false)
    }
    pub fn stale() -> Self {
        Self::new("superseded", "连接配置已变更，本次旧请求已取消", false)
    }
}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for AppError {}

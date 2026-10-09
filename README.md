# 金十阅读器 · Jin10 Reader

安静地看行情、读快讯。使用自备 MCP 凭据的 Windows 桌面阅读器。

[English](README.en.md) · [贡献指南](CONTRIBUTING.md) · [安全说明](SECURITY.md)

版本 **0.1.5**。本项目由独立开发者维护，与金十数据不存在官方合作或背书关系。

## 功能

- 行情快照、最新快讯、深度资讯与财经日历。
- 快讯和资讯搜索、历史分页、资讯详情。
- 浅色、墨色、跟随系统和毛玻璃主题；背景与原生亮度层分别可调。
- 紧凑迷你窗、屏幕边缘吸附、托盘唤回、自定义快捷键。
- 设置采用草稿模式，保存后生效；Token 不返回界面配置读取接口。

新闻与行情只保留在当前进程内存，不包含后台采集、数据库或 RAG 服务。行情每 10 分钟、快讯每 5 分钟、资讯与日历每小时自动刷新；隐藏时暂停，重新显示后恢复。

## 使用

支持 Windows 10/11 x64，需要 Microsoft Edge WebView2。可调毛玻璃额外需要 [Windows App Runtime 2.5.1 x64](https://learn.microsoft.com/en-us/windows/apps/windows-app-sdk/downloads)；缺失时显示提示并回退系统预设。

1. 通过 [金十官方 MCP 页面](https://mcp.jin10.com/app/) 获取自己有权使用的访问凭据。
2. 在设置中填写 MCP 服务地址和 Token，并保存。默认地址为 https://mcp.jin10.com/mcp。
3. 在设置中调整主题、快捷键和启动偏好。毛玻璃的两项不透明度均为 0%–100%，保存后生效，文字保持清晰。

配置保存在 `%APPDATA%\jin10-reader\settings.json`，日志最多保留 7 份。Token 及其配置备份目前为当前用户目录中的明文文件，请勿分享配置目录。程序不内置凭据、不自动上传内容；连接请求会向你配置的端点发送 Token。清除凭据可使用设置中的“清除已存 Token”并保存；撤销远程访问权限需在服务提供方操作。

## 开发与构建

安装 Rust stable 的 MSVC 工具链、Visual Studio C++ Build Tools、Windows SDK、WebView2，以及 Node.js 22+。Node 仅用于开发检查，应用运行不依赖 Node。

```powershell
node tools/check-public.mjs
npm test
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo run --locked
```

项目保留已构建的原生 Acrylic 桥及 SDK 通知。重建执行 `./tools/build-acrylic.ps1`；所需 Microsoft NuGet 包版本与哈希锁定在 `native/sdk-packages.json`。

```powershell
cargo install tauri-cli --version 2.11.4 --locked
./tools/build-release.ps1
node tools/checksums.mjs target/release/bundle/nsis
```

安装包输出到 `target/release/bundle/nsis/`。首次构建需下载依赖，缓存齐备后可使用离线参数。默认测试使用合成数据，不访问真实 MCP；在线测试需要专用凭据及显式开关 `node tools/online-smoke.mjs --allow-network`、`JIN10_RUN_ONLINE=1` 和 `JIN10_TEST_TOKEN`。

## 源码结构

| 路径 | 内容 |
|---|---|
| src/ | Rust/Tauri 桌面与 MCP 核心 |
| dist/ | 手写 HTML、CSS、JavaScript 源码，必须提交 |
| native/ | 官方 Acrylic 桥源码与 SDK 包锁定信息 |
| resources/acrylic/ | 原生 DLL、校验清单、SDK 许可与 NOTICE |
| icons/ | 应用图标 |
| capabilities/、permissions/ | 窗口权限 |
| tests/、tools/ | 离线测试、检查和构建工具 |
| licenses/ | 第三方许可、通知与资产声明 |

## 许可

项目代码采用 [MIT](LICENSE)。第三方依赖及 SDK 仍遵循各自许可，见 [Windows 依赖清单](licenses/third-party-licenses.md)、[原文汇编](licenses/third-party-notices.txt) 和 [资产声明](licenses/ASSETS.md)。代码许可不授予新闻数据、商标或服务再分发权。

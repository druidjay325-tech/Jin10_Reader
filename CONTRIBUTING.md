# 参与贡献

本项目使用 Rust/Tauri 与手写 HTML、CSS、JavaScript。

- 前端 IPC 统一通过 dist/commands.js；远端内容按文本渲染，外链在 Rust 再校验。
- 修改业务行为时添加可复现错误的离线测试，使用合成输入，不提交 Token、配置、日志或新闻数据。
- dist/ 是源码，必须提交。请勿提交 target/、node_modules/、运行数据或旧仓历史。
- 提交前执行 README 中的检查和测试。涉及系统行为时说明 Windows/WebView2 版本、DPI、复现步骤及实际结果。
- 使用 Conventional Commits，说明用户能观察到的变化与验证结果。

安全问题请按 SECURITY.md 私下报告。

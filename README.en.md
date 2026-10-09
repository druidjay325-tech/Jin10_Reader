# Jin10 Reader

A quiet Windows desktop reader for market snapshots, news flashes, articles, and the economic calendar. Bring your own authorized MCP credentials. Version 0.1.5. This is an independent, unofficial client.

[简体中文](README.md) · [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md)

## Features and requirements

Search, pagination, light/dark/system/glass themes, adjustable glass overlay and luminosity, mini view, edge snapping, tray restore, and keyboard shortcuts. News and quotes stay in process memory; there is no collector or content database.

Windows 10/11 x64 and WebView2 are required. Adjustable glass additionally requires Windows App Runtime 2.5.1 x64; without it, the app reports a fallback to the system preset. Credentials are obtained from [Jin10](https://mcp.jin10.com/app/). The default endpoint is https://mcp.jin10.com/mcp.

Settings and their backup contain your Token in plain text under `%APPDATA%\jin10-reader`. Do not share this directory. The configuration API does not return the Token to the UI. Clearing local credentials does not revoke upstream access.

## Build

Install Rust stable with MSVC, Visual Studio C++ Build Tools, Windows SDK, WebView2, and Node.js 22+. Node is needed for development checks, not application runtime.

```powershell
node tools/check-public.mjs
npm test
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo run --locked
```

To rebuild the native bridge, run `./tools/build-acrylic.ps1`. SDK package versions and hashes are pinned in `native/sdk-packages.json`.

To build the Windows installer, install Tauri CLI 2.11.4 with `cargo install tauri-cli --version 2.11.4 --locked`, then run `./tools/build-release.ps1`. Output is under `target/release/bundle/nsis/`. Default tests are offline; live smoke tests require explicit opt-in and dedicated credentials.

## License

Project code is [MIT](LICENSE). Third-party components retain their own terms: [dependency licenses](licenses/third-party-licenses.md), [notices](licenses/third-party-notices.txt), and [asset attribution](licenses/ASSETS.md). Code licensing grants no rights to upstream data, trademarks, or service access.

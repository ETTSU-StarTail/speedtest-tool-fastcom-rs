# speedtest-tool-fastcom-rs
Rust 実装の speedtest_tool_fastcom

## 変更・追加要素

- puppeteer から playwright に変更
  - インストール済み Microsoft Edge を利用するよう変更
- 引数にプロキシ設定を追加
  - --proxy-url
  - --proxy_bypass
  - --proxy_username
  - --proxy_password

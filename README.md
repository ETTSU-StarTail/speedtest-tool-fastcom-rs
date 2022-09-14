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

## Usage

use proxy.

```powershell
path\to\speedtest-tool-fastcom-rs.exe --upload-path <any path> --save-path <any path> --proxy-url <proxy url http(s)//host:port>
```

not use proxy.

```powershell
path\to\speedtest-tool-fastcom-rs.exe --upload-path <any path> --save-path <any path>
```

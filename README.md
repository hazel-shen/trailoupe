# Trailoupe

Trailoupe 是一個在電腦上執行的手機鑑識工具：分析 iPhone 的加密備份與 sysdiagnose 檔案，
比對公開的惡意指標（STIX2 格式），找出間諜軟體可能留下的痕跡。

本工具只用於使用者檢查**自己的**裝置。

> **重要**：Trailoupe 只能告訴你「未發現已知指標」，無法證明裝置是安全的。
> 指標清單只涵蓋已被公開記錄的威脅，未知或新型的間諜軟體不會被偵測到。

## 狀態

專案目前只有骨架，尚未實作任何實際的解析邏輯。所有子指令都會印出「尚未實作」。

## 結構

| Crate | 類型 | 說明 |
| --- | --- | --- |
| `crates/trailoupe-core` | library | 備份 / sysdiagnose 解析、指標載入、時間軸與報告 |
| `crates/trailoupe-cli` | binary（`trailoupe`） | 命令列介面 |

GUI 尚未建立。

## 使用

```sh
cargo run -p trailoupe-cli -- scan-backup <備份目錄>
cargo run -p trailoupe-cli -- scan-sysdiagnose <sysdiagnose.tar.gz>
cargo run -p trailoupe-cli -- update-iocs
```

## 指標檔

本 repo 不包含任何指標檔（STIX2 等）。指標檔一律在執行時從原始發布來源下載，
並遵循各來源自己的授權條款。

## 開發

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

規則：

- 輸入資料（備份、壓縮檔、指標檔）一律視為不可信：解析時不使用 `unwrap` / `expect`，不使用 `unsafe`。
  workspace lint 已設定 `unsafe_code = "forbid"`，並對非測試程式碼禁止 `unwrap_used` / `expect_used`。
- 不要複製或逐行翻譯 MVT（Mobile Verification Toolkit）的程式碼。
- 任何輸出都不得表示「裝置是安全的」。

## 授權

各 crate 採用不同授權：

- **`trailoupe-core`**：MIT 或 Apache-2.0 雙授權（`MIT OR Apache-2.0`），由使用者擇一適用。
  全文見 [LICENSE-MIT](LICENSE-MIT) 與 [LICENSE-APACHE](LICENSE-APACHE)。
  寬鬆授權讓其他專案（包括未來的 GUI 或第三方工具）可以自由引用此函式庫。
- **`trailoupe-cli`**：GNU General Public License v3.0 或更新版本（`GPL-3.0-or-later`）。
  全文見 [LICENSE-GPL](LICENSE-GPL)。

除非另有明確聲明，任何你有意提交以納入本專案的貢獻，都依該檔案所屬 crate 的授權條款授權，
不附加任何額外條款或條件。

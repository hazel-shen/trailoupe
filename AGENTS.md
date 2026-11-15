# Trailoupe — AI Agent 開發規範

## 專案概述
Trailoupe 是在電腦上執行的手機鑑識工具：分析 iPhone 加密備份與 sysdiagnose，
比對公開的惡意指標（STIX2），找出間諜軟體留下的痕跡。只用於使用者檢查自己的裝置。

## 常用指令
- 建置：`cargo build --locked`
- 測試：`cargo test --locked`
- 格式：`cargo fmt --all -- --check`
- Lint：`cargo clippy --all-targets --locked -- -D warnings`
- 漏洞與授權檢查：`cargo audit`、`cargo deny check`

每次完成修改後，上述指令必須全部通過才算完成。

## 架構
- `crates/trailoupe-core`：解析引擎（library），授權 MIT OR Apache-2.0
- `crates/trailoupe-cli`：命令列工具，執行檔名稱 `trailoupe`，授權 GPL-3.0-or-later
- GUI 尚未建立，不要自行新增

core 不得依賴 cli，也不得包含任何 UI 或終端輸出邏輯。

## 必須遵守的規則

### 授權
- 不得複製或逐行翻譯 MVT（Mobile Verification Toolkit）的程式碼；可參考其文件理解檔案格式，但程式碼必須自行撰寫
- 參考 iShutdown（MIT）時，須在檔案開頭註明出處
- core 只能依賴與 MIT / Apache-2.0 相容的套件
- 不得修改 LICENSE 檔案

### 不可信輸入
所有備份、日誌、壓縮檔、指標檔都可能由攻擊者構造。
- 解析輸入的程式碼不得使用 `unwrap`、`expect`、`panic!`、陣列直接索引；一律回傳錯誤
- 不得使用 `unsafe`
- 解壓縮時必須檢查路徑穿越（`../`、絕對路徑、符號連結），並限制單檔與總解壓大小
- 讀取檔案前檢查大小上限，避免一次載入整個大檔

### 隱私
- 唯一允許的網路存取是 `update-iocs` 下載指標檔
- 不得加入遙測、錯誤回報、分析或任何把使用者資料送出本機的功能
- 日誌與錯誤訊息不得印出簡訊內容、通話紀錄等個人資料

### 輸出用語
- 不得輸出「裝置安全」「未感染」等結論，只能說「未發現已知指標」
- 報告須註明：公開指標無法證明裝置未遭入侵

### 資料
- repo 中不得放入任何指標檔（STIX2 等），一律在執行時下載
- 測試用 fixture 必須是人工產生的假資料，不得含真實裝置的資料

## 相依套件
- 新增任何套件前，先說明用途與替代方案，取得我的同意
- 能用標準函式庫或少量程式碼解決的，不新增套件
- 特別說明是否含 `build.rs` 或 proc-macro
- 不使用 git 或本機路徑相依，只用 crates.io
- 不得自行執行 `cargo update`；需要升級時先告訴我原因

## 程式風格
- core 的錯誤型別用 `thiserror`，cli 用 `anyhow`
- 公開 API 必須有 doc comment
- 每個解析器至少有三類測試：正常輸入、格式錯誤、惡意輸入（超大、路徑穿越、截斷）

## 工作流程
- 動手前先簡述計畫；一次只處理一個模組
- commit 訊息使用 Conventional Commits（`feat:`、`fix:`、`test:`…）
- 不要 push，不要修改 CI 中以 commit SHA 鎖定的 action 版本
- 不得為了讓檢查通過而關閉 lint、刪除測試或加上 `#[allow(...)]`；遇到困難先回報

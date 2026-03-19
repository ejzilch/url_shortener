# URL Shortener (Rust)

這是一個學習 Rust 語言實作的短網址系統。

## Project struct
```
url-shortener/
├── Cargo.toml          # 工作區配置
├── src/
│   ├── main.rs         # 程式進入點
│   ├── handlers.rs     # 處理 HTTP Request 的邏輯
│   ├── store.rs        # 資料儲存抽象層
│   ├── lib.rs          # 供測試與外部使用
│   ├── error.rs        # 統一錯誤定義
│   └── utils.rs        # 短碼生成演算法
└── tests/
    └── api_tests.rs    # 整合測試
```
## 技術使用
- 語言: Rust
- Web 框架: Axum
- 存儲: Redis / SQLite (預計)

## 開發進度
- [x] 專案初始化
- [x] 基礎 API 設計
- [ ] 短網址編碼邏輯實作
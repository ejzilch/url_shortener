# URL Shortener (Rust)

這是一個學習 Rust 語言實作的短網址系統。

## Project struct

rust-url-shortener/
├── Cargo.toml          # 工作區配置
├── src/
│   ├── main.rs         # 程式進入點 (設定路由與啟動 Server)
│   ├── handlers.rs     # 處理 HTTP Request 的邏輯 (Shorten, Redirect)
│   ├── store.rs        # 資料儲存抽象層 (In-memory 或 Database)
│   └── utils.rs        # 短碼生成演算法 (Base62/Hash)
└── tests/              # 整合測試
    └── api_tests.rs

## 技術使用
- 語言: Rust
- Web 框架: Axum (預計)
- 存儲: Redis / SQLite (預計)

## 開發進度
- [x] 專案初始化
- [ ] 基礎 API 設計
- [ ] 短網址編碼邏輯實作
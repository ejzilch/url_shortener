use anyhow::Context;
use axum::Router;
use std::net::SocketAddr;

use url_shortener::create_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app: Router = create_app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server 啟動於 http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("無法綁定到指定addr")?;

    axum::serve(listener, app).await.context("伺服器錯誤")?;

    Ok(())
}

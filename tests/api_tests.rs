use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;
// 假設專案名稱為 url_shortener
use url_shortener::create_app;

#[tokio::test]
async fn test_full_cycle() -> anyhow::Result<()> {
    let app = create_app();

    let request = Request::builder()
        .method("POST")
        .uri("/shorten")
        .header("content-type", "application/json")
        .body(Body::from(json!({"url": "https://google.com"}).to_string()))?;

    let response = app.clone().oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await?.to_bytes();
    let body: Value = serde_json::from_slice(&body)?;

    let short_url = body["short_url"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing short_url field"))?;

    let code = short_url
        .split('/')
        .last()
        .ok_or_else(|| anyhow::anyhow!("Invalid short_url format"))?;

    let request = Request::builder()
        .method("GET")
        .uri(format!("/{}", code))
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        response.headers().get("location").unwrap(),
        "https://google.com"
    );

    Ok(())
}

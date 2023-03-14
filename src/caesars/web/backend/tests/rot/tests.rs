use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use backend::routes::app;
use cynic::QueryBuilder;
use serde_json::{from_slice, to_string};
use tower::util::ServiceExt;

use super::{
    graphql::queries::{DecryptArguments, DecryptQuery, EncryptArguments, EncryptQuery},
    schema::{DecryptResponse, EncryptResponse},
};

#[tokio::test]
async fn encrypt() -> Result<()> {
    let app = app().await?;

    let args = EncryptArguments {
        plain: "a".to_string(),
        rotation: 13,
    };
    let query = EncryptQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let encrypt_response: EncryptResponse = from_slice(&resp_byte)?;
    assert_eq!(encrypt_response.data.encrypt.secret, "n");

    Ok(())
}

#[tokio::test]
async fn decrypt() -> Result<()> {
    let app = app().await?;

    let args = DecryptArguments {
        secret: "n".to_string(),
        rotation: 13,
    };
    let query = DecryptQuery::build(args);
    let request = Request::builder()
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .uri("/graphql")
        .body(Body::from(to_string(&query)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let resp_byte = hyper::body::to_bytes(response.into_body()).await?;
    let decrypt_response: DecryptResponse = from_slice(&resp_byte)?;
    assert_eq!(decrypt_response.data.decrypt.plain, "a");

    Ok(())
}

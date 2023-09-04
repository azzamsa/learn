pub mod graphql;
mod schema;
use cynic::QueryBuilder;

use crate::rot::{
    graphql::queries::{DecryptArguments, DecryptQuery, EncryptArguments, EncryptQuery},
    schema::{DecryptResponse, EncryptResponse},
};

pub async fn encrypt(plain: String) -> String {
    let args = EncryptArguments {
        plain,
        rotation: 13,
    };
    let query = EncryptQuery::build(args);
    let response = request(&query).await;
    let response: EncryptResponse = serde_json::from_str(&response).unwrap();
    response.data.encrypt.secret
}

pub async fn decrypt(secret: String) -> String {
    let args = DecryptArguments {
        secret,
        rotation: 13,
    };
    let query = DecryptQuery::build(args);
    let response = request(&query).await;
    let response: DecryptResponse = serde_json::from_str(&response).unwrap();
    response.data.decrypt.plain
}

async fn request<T>(query: &T) -> String
where
    T: serde::Serialize,
{
    let client = reqwest::Client::new();
    client
        .post("http://127.0.0.1:7000/graphql")
        .json(query)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

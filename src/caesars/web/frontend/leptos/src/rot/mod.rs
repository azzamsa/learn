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
    let client = reqwest::Client::new();
    let response: EncryptResponse = client
        .post("http://127.0.0.1:7000/graphql")
        .json(&query)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    response.data.encrypt.secret
}

pub async fn decrypt(secret: String) -> String {
    let args = DecryptArguments {
        secret,
        rotation: 13,
    };
    let query = DecryptQuery::build(args);
    let client = reqwest::Client::new();
    let response: DecryptResponse = client
        .post("http://127.0.0.1:7000/graphql")
        .json(&query)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    response.data.decrypt.plain
}

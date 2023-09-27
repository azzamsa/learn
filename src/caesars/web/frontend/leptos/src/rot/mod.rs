pub mod graphql;
use cynic::QueryBuilder;

use crate::rot::graphql::queries::{
    DecryptArguments, DecryptQuery, EncryptArguments, EncryptQuery,
};

pub async fn encrypt(plain: String) -> String {
    let args = EncryptArguments {
        plain,
        rotation: 13,
    };
    let query = EncryptQuery::build(args);
    let response = request(&query).await;
    let response: cynic::GraphQlResponse<EncryptQuery> = response.json().await.unwrap();
    response.data.unwrap().encrypt.secret
}

pub async fn decrypt(secret: String) -> String {
    let args = DecryptArguments {
        secret,
        rotation: 13,
    };
    let query = DecryptQuery::build(args);
    let response = request(&query).await;
    let response: cynic::GraphQlResponse<DecryptQuery> = response.json().await.unwrap();
    response.data.unwrap().decrypt.plain
}

async fn request<T>(query: &T) -> reqwest::Response
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
}

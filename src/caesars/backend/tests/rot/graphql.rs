mod schema {
    cynic::use_schema!("tests/schema.graphql");
}

#[cynic::schema_for_derives(file = "tests/schema.graphql", module = "schema")]
pub mod queries {

    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "EncryptArguments")]
    pub struct EncryptQuery {
        #[arguments(plain: $plain, rotation : $rotation)]
        pub encrypt: Encrypt,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct EncryptArguments {
        pub plain: String,
        pub rotation: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct Encrypt {
        pub secret: String,
        pub rotation: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "DecryptArguments")]
    pub struct DecryptQuery {
        // #[arguments(secret = &args.secret, rotation = &args.rotation)]
        #[arguments(secret : $secret, rotation: $rotation)]
        pub decrypt: Decrypt,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct DecryptArguments {
        pub secret: String,
        pub rotation: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct Decrypt {
        pub plain: String,
        pub rotation: i32,
    }
}

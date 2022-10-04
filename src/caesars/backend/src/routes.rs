use std::{fs, sync::Arc};

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config,
    config::Config,
    context::ServerContext,
    health, meta, rot, routes,
    schema::{AppSchema, Query},
    Error,
};

pub async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
pub async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub async fn app() -> Result<Router, Error> {
    let config = Arc::new(Config::load()?);

    let meta_service = Arc::new(meta::Service::new());
    let health_service = Arc::new(health::Service::new());
    let rot_service = Arc::new(rot::Service::new());

    let server_context = Arc::new(ServerContext {
        meta_service,
        health_service,
        rot_service,
    });

    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(Arc::clone(&server_context))
        .finish();

    // Export schema to file
    let schema_location = &config.schema_location;
    fs::write(schema_location, &schema.sdl())?;

    #[derive(OpenApi)]
    #[openapi(
        handlers(
            health::resolver::health,
        ),
        components(health::model::Health, health::model::HealthResponse),
        tags(
            (name = "Rust GraphQL", description = "Rust GraphQL Boilerplate 🏗️")
        )
    )]
    struct ApiDoc;

    let mut app = Router::new()
        .route("/graphql", post(routes::graphql_handler))
        .route("/health", get(health::resolver::health));
    if config.env != config::Env::Production {
        app = app
            .route("/playground", get(routes::graphql_playground))
            .merge(
                SwaggerUi::new("/swagger/*tail").url("/api-doc/openapi.json", ApiDoc::openapi()),
            );
    }
    let app = app.layer(Extension(schema));

    Ok(app)
}

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
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config,
    config::Config,
    context::ServerContext,
    domain::{health, meta, rot},
    routes,
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

    let meta_service = Arc::new(meta::Service::new(config.clone()));
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
    if let Some(location) = &config.schema_location {
        fs::write(location, schema.sdl()).map_err(|_| {
            Error::InvalidArgument(format!(
                "GraphQL schema location doesn't exists `{}`",
                &location
            ))
        })?;
        tracing::info!("Wrote GraphQL schema to {}", location);
    }

    #[derive(OpenApi)]
    #[openapi(
        paths(
            health::resolver::health,
        ),
        components(schemas(health::model::Health, health::model::HealthResponse)),
        tags(
            (name = "Rust GraphQL", description = "nROT backend 🔐")
        )
    )]
    struct ApiDoc;

    let cors = CorsLayer::new().allow_origin(Any);
    let mut app = Router::new()
        .route("/graphql", post(routes::graphql_handler))
        .route("/health", get(health::resolver::health))
        .layer(cors);
    if config.env != config::Env::Production {
        app = app
            .route("/playground", get(routes::graphql_playground))
            .merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()));
    }
    let app = app.layer(Extension(schema));

    Ok(app)
}

use crate::routes;
use axum::{
    routing::{get, post},
    Extension, Router, Server,
};
use hyper::{Body, Request, Result};
use sqlx::PgPool;
use std::{net::TcpListener, sync::Arc};
use tower_http::trace::TraceLayer;
use tower_request_id::RequestIdLayer;
use tracing::error_span;
use uuid::Uuid;

pub async fn run(listener: TcpListener, db_pool: PgPool) -> Result<()> {
    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe))
        .layer(Extension(Arc::new(db_pool)))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                // We get the request id from the extensions
                let request_id = Uuid::new_v4();
                // And then we put it along with other information into the `request` span
                error_span!(
                    "request",
                    request_id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            }),
        )
        .layer(RequestIdLayer);

    Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await
}

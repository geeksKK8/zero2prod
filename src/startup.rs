use axum::{
    routing::{get, post},
    Extension, Router, Server,
};
use sqlx::PgPool;

use crate::routes;

use hyper::Result;
use std::{net::TcpListener, sync::Arc};

pub async fn run(listener: TcpListener, db_pool: PgPool) -> Result<()> {
    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe))
        .layer(Extension(Arc::new(db_pool)));

    Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await
}

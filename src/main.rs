use axum::{
    routing::{post}, Router, Extension
};
use db::create_pool;

use std::{net::SocketAddr, sync::Arc};

mod routes;
mod db;

#[tokio::main]
async fn main() {

    let db=match create_pool().await {
        Ok(db) => db,
        Err(error) => panic!("{}", error),
    };

    let app = Router::new()
        .route("/user/auth", post(routes::auth_user))
        .route("/user/create", post(routes::create_user))
        .route("/user/refresh", post(routes::refresh_token))
        .route("/user/logout", post(routes::logout))
        .layer(Extension(Arc::new(db)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use std::time::Duration;

use axum::{
    http::{HeaderValue, Method},
    Router,
};
use member::RawMember;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod member;
mod results;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv()?;

    let host = std::env::var("Host")?;
    let port = std::env::var("Port")?;
    let binded_at = format!("{}:{}", host, port);

    let db_connection_str = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    sqlx::migrate!("./migrations").run(&pool).await?;

    println!("binded at {}", binded_at);

    let state = AppState { pool };

    let app = Router::new()
        .nest("/member", RawMember::routes())
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:4200".parse::<HeaderValue>()?)
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT]),
        )
        .with_state(state);

    let listener = TcpListener::bind(binded_at).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>,
}

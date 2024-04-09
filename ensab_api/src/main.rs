use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    Router,
};

use shared::{get_postgres_pool, Pool, Postgres};
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
    let pool = get_postgres_pool(&db_connection_str).await?;

    println!("binded at {}", binded_at);

    let state = AppState { pool };

    let app = Router::new()
        .nest("/member", member::routes())
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:4200".parse::<HeaderValue>()?)
                .allow_headers([AUTHORIZATION, CONTENT_TYPE])
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

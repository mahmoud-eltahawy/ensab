use std::{collections::HashMap, env};

use axum::{routing::get, Router};
use member::RawMember;

use tokio::net::TcpListener;

mod member;
mod results;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv()?;

    let vars = env::vars();
    let vars = vars
        .filter(|(key, _)| key == "Port" || key == "Host")
        .collect::<HashMap<_, _>>();
    let binded_at = format!(
        "{}:{}",
        vars.get("Host").unwrap(),
        vars.get("Port").unwrap()
    );

    println!("binded at {}", binded_at);

    let app = Router::new()
        .route("/", get(root))
        .nest("/member/", RawMember::routes());
    let listener = TcpListener::bind(binded_at).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

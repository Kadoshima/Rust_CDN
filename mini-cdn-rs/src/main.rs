use tracing::{event, Level};
use std::env;
use dotenvy::dotenv;

fn main() {
    dotenv().expect(".env file not found");
    tracing_subscriber::fmt().init();

    event!(Level::INFO, "std::env::var("ORIGIN_BASE")");
}
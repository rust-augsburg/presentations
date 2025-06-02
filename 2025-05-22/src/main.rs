use std::time::Duration;

use otel::init_tracing_subscriber;
use tracing::{error, info};

mod business;
mod cfg;
mod otel;
mod server;

#[tokio::main]
async fn main() {
    println!("{:#^70}", "");
    println!("{:#^70}", "");
    println!("{:#^70}", "  Starting simple demo program...  ");
    println!("{:#^70}", "");
    println!("{:#^70}", "");
    dotenvy::dotenv().expect("shut down when environmental variables cannot be read");

    let _guard = init_tracing_subscriber();
    let cfg = cfg::Cfg { port: 5173 };

    super_cool_function().await;

    tokio::select! {
        err = server::host_server(cfg) => {
            error!("Web-Server-Error:\n{err:?}");
        }
    }
}

#[tracing::instrument]
async fn super_cool_function() {
    tokio::time::sleep(Duration::from_millis(100)).await;
    info!("Rust Meetup Augsburg");
}

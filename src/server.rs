// TODO:
// 1. create auth middleware that checks for valid jwt
//    - how it'd work is by retrieving jwt attached in request metadata,
//      and get the user associated with the token. the append the user id
//      in the request metadata.
//
mod middleware;
mod proto;
mod services;
mod starknet;
mod supabase;

use std::{
    process::exit,
    sync::{Arc, Mutex},
};

use color_eyre::Report;
use dotenv::dotenv;
use middleware::RequestLoggerLayer;
use services::{
    auth::{AuthServer, AuthService},
    rating::{RatingServer, RatingService},
    service_request::{ServiceRequestServer, ServiceRequestService},
    user::{UserServer, UserService},
};
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::{fmt::time::LocalTime, EnvFilter};

fn register_shutdown_handler() {
    let shutdown_flag = Arc::new(Mutex::new(false));

    ctrlc::set_handler(move || {
        let mut flag = shutdown_flag.lock().unwrap();
        let flag_value = *flag;

        if !flag_value {
            *flag = true;
            info!("press CTRL-C again to exit...");
        } else {
            exit(0)
        }
    })
    .expect("could not register shutdown handler");
}

fn setup() {
    dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    tracing_subscriber::fmt::fmt()
        .with_timer(LocalTime::rfc_3339())
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    register_shutdown_handler();
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup();

    let addr = dotenv::var("SOCKET_ADDRESS")
        .expect("MISSING SOCKET ADDRESS")
        .parse()
        .expect("UNABLE TO PARSE SOKCET ADDRESS STRING");

    info!("Listening on {}", addr);

    Server::builder()
        .layer(RequestLoggerLayer::default())
        .add_service(ServiceRequestServer::new(ServiceRequestService::new()))
        .add_service(RatingServer::new(RatingService::new()))
        .add_service(UserServer::new(UserService::new()))
        .add_service(AuthServer::new(AuthService::new()))
        .serve(addr)
        .await?;

    Ok(())
}

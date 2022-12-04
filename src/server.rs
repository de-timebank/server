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

use dotenv::dotenv;
use middleware::RequestLoggerLayer;
use proto::timebank::user::user_server::UserServer;
use services::{
    auth::{AuthServer, AuthService},
    rating::{RatingServer, RatingService},
    service_request::{ServiceRequestServer, ServiceRequestService},
    user::UserService,
};
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::{fmt::time::LocalTime, EnvFilter};

fn setup() {
    dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    tracing_subscriber::fmt::fmt()
        .with_timer(LocalTime::rfc_3339())
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

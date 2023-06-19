use autometrics::encode_global_metrics;
use axum::{http::StatusCode, routing::get, Router};
use server::{job, MyJobRunner};
use std::net::SocketAddr;
use tonic::transport::Server;

//use server::{hello_world, MyGreeter};

mod server;

#[tokio::main]
async fn main() {
    let grpc_addr = "127.0.0.1:50051".parse().unwrap();
    let web_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    // gRPC server
    let grpc_service = job::job_runner_server::JobRunnerServer::new(MyJobRunner::default());

    tokio::spawn(async move {
        Server::builder()
            .add_service(grpc_service)
            .serve(grpc_addr)
            .await
            .expect("gRPC server failed");
    });

    // Web server with Axum
    let app = Router::new()
        .route("/", get(handler))
        .route("/metrics", get(get_metrics));

    axum::Server::bind(&web_addr)
        .serve(app.into_make_service())
        .await
        .expect("Web server failed");
}

async fn handler() -> &'static str {
    "Hello, World!"
}

pub async fn get_metrics() -> (StatusCode, String) {
    match encode_global_metrics() {
        Ok(metrics) => (StatusCode::OK, metrics),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err)),
    }
}

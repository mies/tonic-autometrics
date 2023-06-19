use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use tonic::{transport::Server, Request, Response, Status};

use autometrics::{
    autometrics, encode_global_metrics, global_metrics_exporter, prometheus_exporter,
};

use job::job_runner_server::{JobRunner, JobRunnerServer};

//use hello_world::{HelloReply, HelloRequest};

//use job::greeter_server::{Greeter, GreeterServer};

use job::{JobReply, JobRequest};

pub mod job {
    tonic::include_proto!("job");
}

#[derive(Debug, Default)]
pub struct MyJobRunner {}

#[tonic::async_trait]
impl JobRunner for MyJobRunner {
    #[autometrics]
    async fn send_job(&self, request: Request<JobRequest>) -> Result<Response<JobReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = job::JobReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

use grpc::grpc_server::Grpc;
// use tonic::{transport::Server, Request, Response, Status};

pub mod grpc {
    tonic::include_proto!("grpc");
}

#[derive(Debug, Default)]
pub struct GrpcServerImpl {}

#[tonic::async_trait]
impl Grpc for GrpcServerImpl {}

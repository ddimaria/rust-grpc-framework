use crate::state::State;
use grpc::grpc_server::Grpc;
use std::marker::{Send, Sync};
// use tonic::{transport::Server, Request, Response, Status};

pub mod grpc {
    tonic::include_proto!("grpc");
}

impl<T: 'static + Send + Sync> grpc::grpc_server::Grpc for State<'static, T> {}

#[derive(Debug, Default)]
pub struct GrpcServerImpl {}

#[tonic::async_trait]
impl Grpc for GrpcServerImpl {}

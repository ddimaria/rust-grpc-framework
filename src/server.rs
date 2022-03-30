use crate::{
    config::CONFIG,
    error::{Error, Result},
    message::{grpc::grpc_server::GrpcServer, GrpcServerImpl},
    middleware::auth::validate_token,
    state::State,
};
use std::marker::{Send, Sync};
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic_health::server::health_reporter;

pub async fn serve<T: 'static + Send + Sync>(state: State<'static, T>) -> Result<()> {
    let addr: SocketAddr = (*CONFIG).server.parse()?;
    let grpc_service = GrpcServer::with_interceptor(state, validate_token);
    let (mut health_reporter, health_service) = health_reporter();
    health_reporter
        .set_serving::<GrpcServer<GrpcServerImpl>>()
        .await;

    log::info!("Started gRPC server on {}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(health_service)
        .add_service(tonic_web::enable(grpc_service))
        .serve(addr)
        .await
        .map_err(|error| Error::Unknown(error.to_string()))
}

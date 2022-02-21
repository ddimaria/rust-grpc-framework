use grpc_framework::error::Result;
use grpc_framework::message::GrpcServerImpl;
use grpc_framework::server::serve;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let state = GrpcServerImpl::default();
    serve(state).await?;

    Ok(())
}

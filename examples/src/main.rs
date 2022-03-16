use grpc_framework::error::Result;
use grpc_framework::server::serve;
use grpc_framework::state::State;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let state = State::<String>::new().await?;
    serve(state).await?;

    Ok(())
}

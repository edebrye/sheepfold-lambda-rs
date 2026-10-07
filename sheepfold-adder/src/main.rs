use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, tracing, Error};
mod http_handler;
use http_handler::function_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let ddb = Client::new(&config);

    run(service_fn(|event| function_handler(&ddb, event))).await
}

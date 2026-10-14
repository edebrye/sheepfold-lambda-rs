use lambda_http::{run, service_fn, tracing, Error};
mod http_handler;
use http_handler::function_handler;
use sheepfold_lambda_utils::get_db;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let db = get_db().await;

    run(service_fn(|event| function_handler(&db, event))).await
}

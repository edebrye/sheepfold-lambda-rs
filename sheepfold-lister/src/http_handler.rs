use aws_sdk_dynamodb::Client;
use lambda_http::{Body, Error, Request, RequestExt, Response};
use serde_dynamo::from_attribute_value;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(
    ddb: &Client,
    event: Request,
) -> Result<Response<Body>, Error> {
    let stage_parameters = event.stage_variables();
    let table_name = stage_parameters
        .first("dynamodb_table_name")
        .expect("Table name not defined.");

    let results: Vec<String> = ddb
        .scan()
        .table_name(table_name)
        .projection_expression("id")
        .send()
        .await?
        .items()
        .to_vec()
        .iter()
        .map(|sheep| from_attribute_value(sheep.get("id").cloned().unwrap()).unwrap())
        .collect();

    let body = serde_json::to_string(&results)?;

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body.into())
        .map_err(Box::new)?)
}

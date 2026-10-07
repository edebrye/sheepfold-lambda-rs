use aws_sdk_dynamodb::Client;
use lambda_http::{Body, Error, Request, RequestExt, Response};
use serde_dynamo::to_item;
use sheepfold_model::{SheepBuilder, SheepColor};
use std::str::FromStr;

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

    let mut sheep_builder = SheepBuilder::new();

    // Extract some useful information from the request
    let parameters = event.query_string_parameters_ref();

    parameters
        .and_then(|params| params.first("name"))
        .and_then(|name| Some(sheep_builder.with_name(name.to_string())));

    parameters
        .and_then(|params| params.first("weight"))
        .and_then(|weight| Some(sheep_builder.with_weight(weight.parse().ok()?)));

    parameters
        .and_then(|params| params.first("color"))
        .and_then(|color| Some(sheep_builder.with_color(SheepColor::from_str(color).ok()?)));

    let sheep = sheep_builder.build();
    let body = serde_json::to_string(&sheep)?;
    let sheep_item = to_item(sheep)?;

    ddb.put_item()
        .table_name(table_name)
        .set_item(Some(sheep_item))
        .send()
        .await?;

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body.into())
        .map_err(Box::new)?;
    Ok(resp)
}

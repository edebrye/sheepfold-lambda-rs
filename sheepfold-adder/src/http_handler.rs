use lambda_http::{Body, Error, Request, RequestExt, Response};
use sheepfold_lambda_utils::{base_response, err_to_response, Db};
use sheepfold_model::{Sheep, SheepColor};
use std::str::FromStr;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(db: &Db, event: Request) -> Result<Response<Body>, Error> {
    let mut sheep_builder = Sheep::create();

    // Extract some useful information from the request
    let parameters = event.query_string_parameters_ref();

    if let Some(name) = parameters.and_then(|params| params.first("name")) {
        sheep_builder = sheep_builder.name(name.to_string());
    }

    if let Some(weight) = parameters.and_then(|params| params.first("weight")) {
        sheep_builder = match weight.parse::<u64>() {
            Err(err) => return Ok(err_to_response(err, 403)),
            Ok(weight) => sheep_builder.weight(weight),
        };
    }

    if let Some(color) = parameters.and_then(|params| params.first("color")) {
        sheep_builder = match SheepColor::from_str(color) {
            Err(err) => return Ok(err_to_response(err, 403)),
            Ok(color) => sheep_builder.color(color),
        };
    }

    let sheep = match sheep_builder.exec(&mut db.clone()).await {
        Err(err) => return Ok(err_to_response(err, 502)),
        Ok(value) => value,
    };

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = base_response()
        .status(200)
        .body(serde_json::to_string(&sheep)?.into())
        .map_err(Box::new)?;
    Ok(resp)
}

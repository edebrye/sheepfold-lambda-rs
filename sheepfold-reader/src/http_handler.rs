use lambda_http::{Body, Error, Request, RequestExt, Response};
use sheepfold_lambda_utils::{base_response, err_to_response, get_id_from_parameters, Db};
use sheepfold_model::Sheep;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(db: &Db, event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    // Extract some useful information from the request
    let id = match get_id_from_parameters(event.path_parameters()) {
        Err(err) => return Ok(err_to_response(err, 403)),
        Ok(value) => value,
    };

    let sheep = match Sheep::get_by_id(&mut db.clone(), id).await {
        Err(err) => return Ok(err_to_response(err, 404)),
        Ok(value) => value,
    };

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    Ok(base_response()
        .status(200)
        .body(serde_json::to_string(&sheep)?.into())
        .map_err(Box::new)?)
}

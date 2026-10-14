use lambda_http::{Body, Error, Request, RequestExt, Response};
use serde_json::json;
use sheepfold_lambda_utils::{base_response, err_to_response, get_id_from_parameters, Db};
use sheepfold_model::Sheep;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(db: &Db, event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let id = match get_id_from_parameters(event.path_parameters()) {
        Err(err) => return Ok(err_to_response(err, 403)),
        Ok(value) => value,
    };

    if let Err(err) = Sheep::delete_by_id(&mut db.clone(), id).await {
        return Ok(err_to_response(err, 404));
    };

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = base_response()
        .status(200)
        .body(
            json!({"deleted": true, "id": id.to_string()})
                .to_string()
                .into(),
        )
        .map_err(Box::new)?;
    Ok(resp)
}

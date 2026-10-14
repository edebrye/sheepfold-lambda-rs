use lambda_http::{Body, Error, Request, Response};
use sheepfold_lambda_utils::{base_response, Db};
use sheepfold_model::Sheep;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(db: &Db, _: Request) -> Result<Response<Body>, Error> {
    let sheep_ids: Vec<String> = Sheep::all()
        .exec(&mut db.clone())
        .await?
        .iter()
        .map(|sheep| sheep.get_id().to_string())
        .collect();

    let body = serde_json::to_string(&sheep_ids)?;

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    Ok(base_response()
        .status(200)
        .body(body.into())
        .map_err(Box::new)?)
}

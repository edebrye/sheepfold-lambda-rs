use aws_sdk_dynamodb::{types::ReturnValue, Client};
use lambda_http::{Body, Error, Request, RequestExt, Response};
use serde_dynamo::{from_item, to_attribute_value};
use serde_json::{Map, Value};
use sheepfold_model::Sheep;

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

    // Extract some useful information from the request
    let parameters = event.path_parameters();
    let id = parameters.first("id");

    if id.is_none() {
        return Ok(Response::builder()
            .status(403)
            .body(format!("Missing parameter 'id'").into())
            .map_err(Box::new)?);
    }

    let id = id.unwrap();

    let response = ddb
        .delete_item()
        .table_name(table_name)
        .key("id", to_attribute_value(id)?)
        .return_values(ReturnValue::AllOld)
        .send()
        .await?;

    if response.attributes().is_none() {
        let mut message = Value::Object(Map::new());
        message.as_object_mut().unwrap().insert(
            "message".into(),
            Value::String(format!("No sheep for id {id}.")),
        );

        return Ok(Response::builder()
            .status(404)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&message)?.into())
            .map_err(Box::new)?);
    }

    let deleted_sheep: Sheep = from_item(response.attributes().cloned().unwrap())?;
    let mut body = Map::new();

    body.insert("deleted".into(), Value::Bool(true));
    body.insert("old".into(), serde_json::to_value(deleted_sheep)?);

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&Value::Object(body))?.into())
        .map_err(Box::new)?;
    Ok(resp)
}

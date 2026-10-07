locals {
  # extract all path parameters from the path (ex: "/sheep/{id}/{name}" => ["id", "name"], "/sheep" => [])
  path_parameters = flatten(regexall("{([[:alnum:]]+)}", var.api_path))
  method_path_parameters = {
    for parameter in local.path_parameters : "method.request.path.${parameter}" => true
  }
  integration_path_parameters = {
    for parameter in local.path_parameters : "integration.request.path.${parameter}" => "method.request.path.${parameter}"
  }
}

resource "aws_api_gateway_method" "this" {
  rest_api_id        = var.api_id
  resource_id        = var.api_resource_id
  http_method        = var.api_method
  authorization      = "NONE"
  request_parameters = local.method_path_parameters
}

resource "aws_cloudwatch_log_group" "this" {
  name              = "/aws/lambda/${aws_lambda_function.this.function_name}"
  retention_in_days = 7
}

resource "aws_lambda_function" "this" {
  function_name = var.name
  role          = var.role_arn
  architectures = ["arm64"]
  runtime       = "provided.al2023"
  filename      = "${path.module}/../../../../../../target/lambda/${var.name}/bootstrap.zip"
  code_sha256   = filebase64sha256("${path.module}/../../../../../../target/lambda/${var.name}/bootstrap.zip")
  handler       = "bootstrap"
  memory_size   = 128
  timeout       = 30
}

resource "aws_lambda_permission" "this" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.this.function_name
  principal     = "apigateway.amazonaws.com"

  # More: http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-control-access-using-iam-policies-to-invoke-api.html
  source_arn = "arn:aws:execute-api:${var.region}:${var.account_id}:${var.api_id}/*/${aws_api_gateway_method.this.http_method}${var.api_path}"
}

resource "aws_api_gateway_integration" "this" {
  rest_api_id             = var.api_id
  resource_id             = var.api_resource_id
  http_method             = aws_api_gateway_method.this.http_method
  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.this.invoke_arn
  timeout_milliseconds    = 29000
  request_parameters      = local.integration_path_parameters
}

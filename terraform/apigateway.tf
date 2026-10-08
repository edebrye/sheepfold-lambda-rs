data "aws_caller_identity" "current" {}
data "aws_region" "current" {}

resource "aws_api_gateway_rest_api" "sheepfold" {
  name = local.project
  endpoint_configuration {
    types = ["REGIONAL"]
  }
}

resource "aws_api_gateway_resource" "sheep" {
  rest_api_id = aws_api_gateway_rest_api.sheepfold.id
  parent_id   = aws_api_gateway_rest_api.sheepfold.root_resource_id
  path_part   = "sheep"
}

resource "aws_api_gateway_resource" "sheep_id" {
  rest_api_id = aws_api_gateway_rest_api.sheepfold.id
  parent_id   = aws_api_gateway_resource.sheep.id
  path_part   = "{id}"
}

resource "aws_api_gateway_deployment" "this" {
  rest_api_id = aws_api_gateway_rest_api.sheepfold.id
  triggers = {
    redeploy = join("-", [
      filesha1("./apigateway.tf"),
      filesha1("./lambda.tf"),
    ])
  }

  depends_on = [
    module.lister,
    module.adder,
    module.reader,
    module.remover,
  ]

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_api_gateway_stage" "v1" {
  rest_api_id   = aws_api_gateway_rest_api.sheepfold.id
  deployment_id = aws_api_gateway_deployment.this.id
  stage_name    = "v1"
  variables = {
    dynamodb_table_name = aws_dynamodb_table.sheepfold.name
  }
}

resource "aws_api_gateway_method_settings" "v1" {
  rest_api_id = aws_api_gateway_rest_api.sheepfold.id
  stage_name  = aws_api_gateway_stage.v1.stage_name
  method_path = "*/*"

  settings {
    logging_level = "OFF" #"ERROR"
  }
}

resource "aws_cloudwatch_log_group" "apigw_log_group" {
  name              = "API-Gateway-Execution-Logs_${aws_api_gateway_rest_api.sheepfold.id}/${aws_api_gateway_stage.v1.stage_name}"
  retention_in_days = 7
}

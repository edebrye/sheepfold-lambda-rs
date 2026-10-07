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

resource "aws_api_gateway_deployment" "main" {
  rest_api_id = aws_api_gateway_rest_api.sheepfold.id
  triggers = {
    redeploy = join("-", [
      filesha1("./apigateway.tf"),
      filesha1("./lambda.tf"),
    ])
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_api_gateway_stage" "main" {
  rest_api_id   = aws_api_gateway_rest_api.sheepfold.id
  deployment_id = aws_api_gateway_deployment.main.id
  stage_name    = "main"
  variables = {
    dynamodb_table_name = aws_dynamodb_table.sheepfold.name
  }
}

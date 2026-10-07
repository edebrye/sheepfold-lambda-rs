data "aws_iam_policy_document" "assume_role" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
  }
}

data "aws_iam_policy_document" "execution" {
  statement {
    effect    = "Allow"
    resources = [aws_dynamodb_table.sheepfold.arn]
    actions = [
      "dynamodb:*Item",
      "dynamodb:DescribeTable",
      "dynamodb:Scan",
      "dynamodb:Query",
    ]
  }
}

resource "aws_iam_role" "lambda" {
  name               = "${local.project}-lambda-execution-role"
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
}

resource "aws_iam_policy" "sheep" {
  name   = "${local.project}-lambda-execution-policy"
  policy = data.aws_iam_policy_document.execution.json
}

resource "aws_iam_role_policy_attachment" "lambda_execution_custom" {
  role       = aws_iam_role.lambda.name
  policy_arn = aws_iam_policy.sheep.arn
}

resource "aws_iam_role_policy_attachment" "lambda_execution_basic" {
  role       = aws_iam_role.lambda.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

module "lister" {
  source = "./modules/lambda"

  name            = "${local.project}-lister"
  api_resource_id = aws_api_gateway_resource.sheep.id
  api_path        = aws_api_gateway_resource.sheep.path
  api_method      = "GET"

  account_id     = data.aws_caller_identity.current.account_id
  region         = data.aws_region.current.region
  role_arn       = aws_iam_role.lambda.arn
  dynamodb_table = aws_dynamodb_table.sheepfold.name
  api_id         = aws_api_gateway_rest_api.sheepfold.id
}

module "reader" {
  source = "./modules/lambda"

  name            = "${local.project}-reader"
  api_resource_id = aws_api_gateway_resource.sheep_id.id
  api_path        = aws_api_gateway_resource.sheep_id.path
  api_method      = "GET"

  account_id     = data.aws_caller_identity.current.account_id
  region         = data.aws_region.current.region
  role_arn       = aws_iam_role.lambda.arn
  dynamodb_table = aws_dynamodb_table.sheepfold.name
  api_id         = aws_api_gateway_rest_api.sheepfold.id
}

module "adder" {
  source = "./modules/lambda"

  name            = "${local.project}-adder"
  api_resource_id = aws_api_gateway_resource.sheep.id
  api_path        = aws_api_gateway_resource.sheep.path
  api_method      = "POST"

  account_id     = data.aws_caller_identity.current.account_id
  region         = data.aws_region.current.region
  role_arn       = aws_iam_role.lambda.arn
  dynamodb_table = aws_dynamodb_table.sheepfold.name
  api_id         = aws_api_gateway_rest_api.sheepfold.id
}

module "remover" {
  source = "./modules/lambda"

  name            = "${local.project}-remover"
  api_resource_id = aws_api_gateway_resource.sheep_id.id
  api_path        = aws_api_gateway_resource.sheep_id.path
  api_method      = "DELETE"

  account_id     = data.aws_caller_identity.current.account_id
  region         = data.aws_region.current.region
  role_arn       = aws_iam_role.lambda.arn
  dynamodb_table = aws_dynamodb_table.sheepfold.name
  api_id         = aws_api_gateway_rest_api.sheepfold.id
}

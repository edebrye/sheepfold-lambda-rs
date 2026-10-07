resource "aws_dynamodb_table" "sheepfold" {
  name         = local.project
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "id"

  attribute {
    name = "id"
    type = "S"
  }
}

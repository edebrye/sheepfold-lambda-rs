locals {
  project = "sheepfold"
  state_bucket = "${local.project}-terraform-state"
  state_key = "terraform.tfstate"
}

remote_state {
  backend = "s3"
  generate = {
    path = "backend.tf"
    if_exists = "overwrite_terragrunt"
  }
  config = {
    bucket              = local.state_bucket
    key                 = local.state_key
    region              = "ca-central-1"
    use_lockfile        = true
  }
}

generate "locals" {
  path = "locals.tf"
  if_exists = "overwrite"
  contents = <<EOF
locals {
  project = "${local.project}"
  state_location = "s3://${local.state_bucket}/${local.state_key}"
  state_bucket = "${local.state_bucket}"
}
EOF
}

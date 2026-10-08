terraform {
  required_providers {
    aws = {
      version = "~> 6.0"
    }
  }
}

provider "aws" {
  default_tags {
    tags = merge(var.tags, {
      Project        = local.project
      TerraformState = local.state_location
    })
  }

}

variable "tags" {
  type = map(string)
}

output "api_url" {
  value = aws_api_gateway_stage.v1.invoke_url
}

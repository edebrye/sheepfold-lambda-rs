# creates state bucket
tf-bootstrap:
	terragrunt backend bootstrap

# Compile rust code and produce a zip file per lambda
build:
	cargo lambda build --workspace --arm64 --release --output-format zip

# Creates all infrastructure on AWS (DynamoDB, Lambdas, IAM, API Gateway)
tf-apply: build
	terragrunt --working-dir=terraform apply

# deploy functions without all terraform hassle
deploy: build
	cargo lambda deploy sheepfold-adder
	cargo lambda deploy sheepfold-lister
	cargo lambda deploy sheepfold-reader
	cargo lambda deploy sheepfold-remover

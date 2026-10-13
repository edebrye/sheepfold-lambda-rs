# Compile rust code and produce a zip file per lambda
build:
	cargo lambda build --workspace --arm64 --release --output-format zip

# Creates all infrastructure on AWS (DynamoDB, Lambdas, IAM, API Gateway)
tf-apply: build
	terragrunt run --working-dir=terraform --backend-bootstrap -- apply

# deploy functions without all terraform hassle
deploy: build
	cargo lambda deploy sheepfold-adder
	cargo lambda deploy sheepfold-lister
	cargo lambda deploy sheepfold-reader
	cargo lambda deploy sheepfold-remover

test:
	hurl \
	--test \
	--variable base_url=$(shell terragrunt --working-dir=terraform output api_url | jq -r) \
	basic.hurl

tf-destroy:
	terragrunt run --working-dir=terraform -- apply -destroy

TERRAFORM_STATE_BUCKET := sheepfold-terraform-state

tf-clean: tf-destroy
	aws s3api delete-objects \s
	    --bucket $(TERRAFORM_STATE_BUCKET) \
		--delete "$$(aws s3api list-object-versions \
		            --bucket $(TERRAFORM_STATE_BUCKET) \
				    --output json | jq -c '{Objects: [((.Versions + .DeleteMarkers)| .[]) | {Key:.Key, VersionId : .VersionId}], Quiet: false}')"
	aws s3 rb s3://$(TERRAFORM_STATE_BUCKET)

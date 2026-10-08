# Sheepfold

A REST API to perform CR(U)D operations on Sheeps. The goal is to play with AWS Lambdas coded in Rust.

An API Gateway REST seats in the front, a DynamoDB table seats in the back for persistence.

4 Lambdas in Rust to create, retrieve and delete sheeps.

## Tooling

- [Mise](https://mise.jdx.dev/) to handle tool versions (terraform and rust)
- [Cargo Lambda](https://www.cargo-lambda.info/) to build and deploy Lambdas coded in Rust
- [Terragrunt](https://terragrunt.com/) just because it bootstraps the terraform state bucket, keeps my tags DRY and have auto-init
- [Hurl](https://hurl.dev/docs/installation.html) to run a small test suite similar to a Postman collection (but fast and small to write and execute)

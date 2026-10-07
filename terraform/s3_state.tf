resource "aws_s3_bucket_lifecycle_configuration" "this" {
  bucket = local.state_bucket
  rule {
    id     = "delete-old-versions"
    status = "Enabled"
    noncurrent_version_expiration {
      noncurrent_days = 2
    }
    expiration {
      expired_object_delete_marker = true
    }
    abort_incomplete_multipart_upload {
      days_after_initiation = 2
    }
  }
}

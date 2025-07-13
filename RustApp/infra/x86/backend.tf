terraform {
  backend "s3" {
    bucket         = "rust-app-bucket"
    key            = "terraform/state"
    region         = "ap-southeast-1"
  }
}

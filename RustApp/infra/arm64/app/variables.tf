variable "region" {
  description = "AWS region"
  type        = string
  default     = "ap-southeast-1"
}

variable "tags" {
  type = map(string)
  default = {
    Name        = "my-instance"
    Environment = "dev"
    Owner       = "Aditya Kristianto"
    Terraform   = "true"
  }
}
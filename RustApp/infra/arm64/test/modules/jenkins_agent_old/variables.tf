variable "private_subnet_ids" {
  description = "The value of the private subnets ID EC2"
  type        = list(string)
}

variable "vpc_id" {
  description = "The value of the VPC ID"
  type        = string
}
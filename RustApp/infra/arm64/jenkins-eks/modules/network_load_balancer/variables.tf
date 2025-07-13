variable "aws_acm_certificate_arn" {
  description = "The value of the AWS Certificate ARN"
  type        = string
}

variable "vpc_id" {
  description = "The value of the VPC ID"
  type        = string
}

variable "private_subnets" {
  description = "The value of the private subnets EKS Cluster"
  type        = list(string)
}

variable "public_subnets" {
  description = "The value of the public subnets EKS Cluster"
  type        = list(string)
}

variable "security_groups" {
  description = "The value of the Security Groups"
  type        = list(string)
}

variable "nlb_eip" {
  description = "The value of the Network Load Balancer Elastic IP"
  type        = any
}
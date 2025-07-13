variable "vpc_id" {
  description = "The value of the VPC ID"
  type        = string
}

variable "nat_gateway_id" {
    description = "The value of the NAT Gateway ID"
    type        = string
}

variable "internet_gateway_id" {
  description = "The value of the Internet Gateway ID"
    type        = string
}

variable "private_subnet" {
  description = "The value of the private subnet EKS Cluster"
  type        = string
}

variable "public_subnet" {
  description = "The value of the public subnet EKS Cluster"
  type        = string
}
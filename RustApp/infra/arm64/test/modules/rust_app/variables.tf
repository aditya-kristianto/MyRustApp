variable "cidr_blocks" {
  description = "The value of the CIDR Blocks"
  type        = list(string)
}

variable "iam_instance_profile_name" {
  description = "The value of the iam instance profile name"
  type        = string
}

variable "key_name" {
  description = "The value of the key name"
  type        = string
}

variable "postgres_instance_id" {
  description = "The value of the Instance ID Postgres EC2"
  type        = string
}

variable "postgres_private_ip" {
  description = "The value of the Private IP Postgres EC2"
  type        = string
}

variable "private_subnet_id" {
  description = "The value of the private subnet ID EC2"
  type        = string
}

variable "vpc_id" {
  description = "The value of the VPC ID"
  type        = string
}
variable "vpc_id" {
  description = "The value of the VPC ID"
  type        = string
}

variable "key_name" {
  description = "The value of the key name"
  type        = string
}

variable "private_ap_southeast_1a_cidr_block" {
  description = "The value of the private subnet ID"
  type        = string
}

variable "private_ap_southeast_1b_cidr_block" {
  description = "The value of the private subnet ID"
  type        = string
}

variable "private_ap_southeast_1c_cidr_block" {
  description = "The value of the private subnet ID"
  type        = string
}

variable "public_subnet_id" {
  description = "The value of the public subnet ID EC2"
  type        = string
}

variable "iam_instance_profile_name" {
  description = "The value of the iam instance profile name"
  type        = string
}

variable "cidr_blocks" {
  description = "The value of the CIDR Blocks"
  type        = list(string)
}
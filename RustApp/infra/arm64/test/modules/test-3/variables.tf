variable "private_subnet_ids" {
  description = "The value of the private subnets ID"
  type        = list(string)
}

variable "private_ap_southeast_1a" {
  description = "The value of the private subnet ID"
  type        = string
}

variable "private_ap_southeast_1b" {
  description = "The value of the private subnet ID"
  type        = string
}

variable "private_ap_southeast_1c" {
  description = "The value of the private subnet ID"
  type        = string
}

variable "public_ap_southeast_1a" {
  description = "The value of the public subnet ID"
  type        = string
}

variable "public_ap_southeast_1b" {
  description = "The value of the public subnet ID"
  type        = string
}

variable "public_ap_southeast_1c" {
  description = "The value of the public subnet ID"
  type        = string
}

variable "vpc_id" {
  description = "The value of the VPC ID"
  type        = string
}
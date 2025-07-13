variable "vpc_id" {
  description = "The value of the VPC ID"
  type        = string
}

variable "bastion_host_cidr_blocks" {
  description = "The value of the Bastion Host CIDR Blocks"
  type        = list(string)
}

variable "portainer_cidr_blocks" {
  description = "The value of the Portainer CIDR Blocks"
  type        = list(string)
}
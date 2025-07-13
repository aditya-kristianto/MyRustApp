variable "private_subnet_id" {
  description = "The value of the private subnet ID EC2"
  type        = string
}

variable "public_subnet_id" {
  description = "The value of the public subnet ID EC2"
  type        = string
}

variable "key_name" {
  description = "The value of the key name"
  type        = string
}

variable "iam_instance_profile_name" {
  description = "The value of the iam instance profile name"
  type        = string
}

variable "bastion_host_security_group_ids" {
  description = "The value of the VPC Security Groups"
  type        = list(string)
}

variable "portainer_security_group_ids" {
  description = "The value of the VPC Security Groups"
  type        = list(string)
}
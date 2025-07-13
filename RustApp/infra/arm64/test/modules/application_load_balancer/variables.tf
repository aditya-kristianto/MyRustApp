variable "public_subnets" {
  description = "The value of the public subnets ID"
  type        = list(string)
}

variable "vpc_id" {
  description = "The value of the VPC ID"
  type        = string
}

variable "portainer_private_ip" {
  description = "The value of the private ip of Portainer Instance"
  type        = string
}

variable "pgadmin_private_ip" {
  description = "The value of the private ip of PGAdmin Instance"
  type        = string
}

variable "rust_app_private_ip" {
  description = "The value of the private ip of Rust App Instance"
  type        = string
}

variable "jenkins_master_private_ip" {
  description = "The value of the private ip of Jenkins Master Instance"
  type        = string
}

variable "jenkins_master_target_id" {
  description = "The value of the id of Jenkins Master Instance"
  type        = string
}

variable "portainer_target_id" {
  description = "The value of the id of Portainer Instance"
  type        = string
}

variable "certificate_arn" {
  description = "The value of the Certificate ARN"
  type        = string
}
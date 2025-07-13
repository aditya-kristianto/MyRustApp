variable "aws_region" {
  description = "AWS region to deploy resources."
  type        = string
}

variable "jenkins_master_ec2_security_group_id" {
  description = "The value of the VPC Security Group ID"
  type        = string
}

variable "jenkins_agent_ec2_security_group_id" {
  description = "The value of the VPC Security Group ID"
  type        = string
}

variable "portainer_ec2_security_group_id" {
  description = "The value of the VPC Security Group ID"
  type        = string
}

variable "subnet_ids" {
  description = "The list value of the VPC Subnet ID"
  type        = list(string)
}

variable "vpc_id" {
  description = "The value of the VPC ID"
  type        = string
}

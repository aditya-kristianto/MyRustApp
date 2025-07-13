variable "vpc_id" {
  description = "The value of the VPC ID"
  type        = string
}

variable "jenkins_agent_ec2_security_group_name" {
  description = "Name of the security group for Jenkins Agent EC2."
  type        = string
}

variable "jenkins_master_ec2_security_group_name" {
  description = "Name of the security group for Jenkins Master EC2."
  type        = string
}

variable "portainer_ec2_security_group_name" {
  description = "Name of the security group for Portainer EC2."
  type        = string
}

# variable "jenkins_master_security_group_id" {
#   description = "The value of the Security Group ID."
#   type        = string
# }
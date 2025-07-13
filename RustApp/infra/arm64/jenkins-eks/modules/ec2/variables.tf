variable "eks_cluster_name" {
  description = "The value of the EKS Cluster Name"
  type        = string
}

variable "jenkins_agent_iam_instance_profile" {
  description = "The value of the IAM Instance Profile"
  type        = string
}

variable "jenkins_master_iam_instance_profile" {
  description = "The value of the IAM Instance Profile"
  type        = string
}

variable "portainer_iam_instance_profile" {
  description = "The value of the IAM Instance Profile"
  type        = string
}

variable "key_name" {
  description = "The value of the Key Name"
  type        = string
}

variable "bastion_key_name" {
  description = "The value of the Key Name"
  type        = string
}

variable "subnet_id" {
  description = "The value of the Subnet ID"
  type        = string
}

variable "public_subnet_id" {
  description = "The value of the Subnet ID"
  type        = string
}

variable "bastion_host_security_group_id" {
  description = "The value of the VPC Security Group ID"
  type        = string
}

variable "jenkins_master_security_group_id" {
  description = "The value of the VPC Security Group ID"
  type        = string
}

variable "jenkins_agent_security_group_id" {
  description = "The value of the VPC Security Group ID"
  type        = string
}

variable "portainer_security_group_id" {
  description = "The value of the VPC Security Group ID"
  type        = string
}

variable "use_managed_node_group" {
  type = bool
}
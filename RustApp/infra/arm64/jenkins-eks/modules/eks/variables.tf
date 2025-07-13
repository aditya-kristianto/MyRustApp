variable "launch_template_id" {
  description = "The value of the EC2 Launch Template ID"
  type        = string
}

variable "launch_template_name" {
  description = "The value of the EC2 Launch Template Name"
  type        = string
}

variable "role_arn" {
  description = "The value of the IAM Role ARN"
  type        = string
}

variable "subnet_ids" {
  description = "The value of the private subnets EKS Cluster"
  type        = list(string)
}

variable "vpc_id" {
  description = "The value of the VPC ID"
  type        = string
}

variable "eks_worker_nodes_policy" {
  type        = any
  description = "IAM policy attachment for EKS worker nodes."
}

variable "eks_cni_policy" {
  type        = any
  description = "IAM policy attachment for EKS CNI."
}

variable "ec2_container_registry_read_only" {
  type        = any
  description = "IAM policy attachment for EC2 container registry read-only."
}

variable "jenkins_agent_ec2_security_group_id" {
  description = "The value of the VPC Security Group ID"
  type        = string
}

variable "use_managed_node_group" {
  type = bool
}
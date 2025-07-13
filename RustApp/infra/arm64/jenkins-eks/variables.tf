variable "aws_region" {
  description = "AWS region to deploy resources."
  type        = string
  default     = "ap-southeast-1"
}

variable "zone_id" {
  description = "AWS Zone ID to deploy resources."
  type        = string
  default     = "Z0148722AZOXRNZ0DFO2"
}

variable "cluster_name" {
  description = "Name of the EKS cluster."
  type        = string
  default     = "jenkins-eks-cluster"
}

variable "node_group_name" {
  description = "Name of the EKS node group."
  type        = string
  default     = "jenkins-node-group"
}

variable "desired_capacity" {
  description = "Desired number of worker nodes."
  type        = number
  default     = 1
}

variable "max_capacity" {
  description = "Maximum number of worker nodes."
  type        = number
  default     = 2
}

variable "min_capacity" {
  description = "Minimum number of worker nodes."
  type        = number
  default     = 1
}

variable "jenkins_agent_ec2_security_group_name" {
  description = "Name of the security group for Jenkins Agent EC2."
  type        = string
  default     = "jenkins-agent-sg"
}

variable "jenkins_master_ec2_security_group_name" {
  description = "Name of the security group for Jenkins Master EC2."
  type        = string
  default     = "jenkins-master-sg"
}

variable "portainer_ec2_security_group_name" {
  description = "Name of the security group for Jenkins Master EC2."
  type        = string
  default     = "portainer-sg"
}

variable "eks_cluster_version" {
  description = "Kubernetes version for EKS cluster."
  type        = string
  default     = "1.27"
}

variable "eks_node_group_name" {
  description = "Name of the EKS managed node group."
  type        = string
  default     = "jenkins-node-group"
}

variable "eks_node_instance_type" {
  description = "Instance type for EKS worker nodes."
  type        = string
  default     = "t3.medium"
}

variable "eks_desired_capacity" {
  description = "Desired number of EKS worker nodes."
  type        = number
  default     = 1
}

variable "eks_max_capacity" {
  description = "Maximum number of EKS worker nodes."
  type        = number
  default     = 3
}

variable "eks_min_capacity" {
  description = "Minimum number of EKS worker nodes."
  type        = number
  default     = 1
}

variable "eks_key_name" {
  description = "SSH key name for EKS worker nodes."
  type        = string
  default     = "your-ssh-key"  # Replace with your key name
}

variable "jenkins_instance_type" {
  description = "EC2 instance type for Jenkins master."
  type        = string
  default     = "t4g.medium"
}

variable "jenkins_ami" {
  description = "AMI ID for Jenkins master EC2 instance."
  type        = string
  # You can specify a default or override it during Terraform apply.
  default     = "ami-094ebf7c110197581"  # Example: Amazon Linux 2
}

variable "ssh_key_name" {
  description = "SSH key name for EC2 instance."
  type        = string
  default     = "your-ssh-key"  # Replace with your key name
}

variable "use_managed_node_group" {
  type    = bool
  default = false  # Set this to false for unmanaged node group using launch template
}
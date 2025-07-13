variable "instance_type" {
  description = "Type of instance"
  type        = string
  default     = "t2.micro"
}

# Declare the cluster name variable
variable "cluster_name" {
  description = "Name of the EKS cluster"
  type        = string
  default     = "example-cluster"
}

# Declare the subnet IDs for the EKS cluster
variable "subnet_ids" {
  description = "List of subnet IDs for the EKS cluster"
  type        = list(string)
}

# Declare other variables as needed (desired size, min size, etc.)
variable "desired_size" {
  description = "Desired number of worker nodes"
  type        = number
  default     = 2
}

variable "min_size" {
  description = "Minimum number of worker nodes"
  type        = number
  default     = 1
}

variable "max_size" {
  description = "Maximum number of worker nodes"
  type        = number
  default     = 4
}

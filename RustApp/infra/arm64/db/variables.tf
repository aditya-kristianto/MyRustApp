variable "region" {
  description = "AWS region"
  type        = string
  default     = "ap-southeast-1"
}

# Specify the existing VPC by its ID
variable "vpc_id" {
  description = "The ID of the existing VPC"
  type        = string
  default     = "vpc-0bd8aa2ff87fd5358"
}

# Specify the existing subnet ID(s)
variable "subnet_id" {
  description = "The ID of the existing subnet in the VPC"
  type        = string
  default     = "subnet-0b4641aa76c7b23ac"
}

variable "ec2_tags" {
  type = map(string)
  default = {
    Name        = "Postgres DB Instance"
    Environment = "dev"
    Owner       = "Aditya Kristianto"
    Terraform   = "true"
  }
}

variable "vpc_tags" {
  type = map(string)
  default = {
    Name        = "My VPC"
    Environment = "dev"
    Owner       = "Aditya Kristianto"
    Terraform   = "true"
  }
}

variable "sg_tags" {
  type = map(string)
  default = {
    Name        = "Postgres DB Security Group"
    Environment = "dev"
    Owner       = "Aditya Kristianto"
    Terraform   = "true"
  }
}

variable "subnet_tags" {
  type = map(string)
  default = {
    Name        = "My Subnet"
    Environment = "dev"
    Owner       = "Aditya Kristianto"
    Terraform   = "true"
  }
}
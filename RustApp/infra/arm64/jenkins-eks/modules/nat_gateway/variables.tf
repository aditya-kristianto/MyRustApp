variable "public_subnet" {
  description = "The value of the public subnet EKS Cluster"
  type        = string
}

variable "nat_eip_id" {
    description = "The value of the NAT Elastic IP ID"
    type        = string
}
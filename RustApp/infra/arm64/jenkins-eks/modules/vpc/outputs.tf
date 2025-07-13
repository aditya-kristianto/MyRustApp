output "vpc_id" {
  value = aws_vpc.my_vpc.id
  description = "The ID of the VPC."
}

# output "igw_id" {
#   value = module.vpc.igw_id
#   description = "The ID of the Internet Gateway."
# }

# output "public_subnets" {
#   value = module.vpc.public_subnets
#   description = "The Public Subnets of the VPC."
# }

# output "private_subnets" {
#   value = module.vpc.private_subnets
#   description = "The Private Subnets of the VPC."
# }
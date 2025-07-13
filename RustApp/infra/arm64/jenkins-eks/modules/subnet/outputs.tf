output "public_subnets" {
  value = [
    aws_subnet.public_subnet_1.id,
    aws_subnet.public_subnet_2.id,
    aws_subnet.public_subnet_3.id
  ]
  description = "The Public Subnets of the VPC."
}

output "private_subnets" {
  value = [
    aws_subnet.private_subnet_1.id, 
    aws_subnet.private_subnet_2.id,
    aws_subnet.private_subnet_3.id
  ]
  description = "The Private Subnets of the VPC."
}
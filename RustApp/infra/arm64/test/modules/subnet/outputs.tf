output "private_subnets" {
  value = [
    aws_subnet.private_subnet_1.id, 
    aws_subnet.private_subnet_2.id,
    aws_subnet.private_subnet_3.id
  ]
  description = "The Private Subnets of the VPC."
}

output "private_subnet_1_id" {
    value = aws_subnet.private_subnet_1.id
    description = "The Private Subnet 1 of the VPC."
}

output "private_subnet_2_id" {
    value = aws_subnet.private_subnet_2.id
    description = "The Private Subnet 2 of the VPC."
}

output "private_subnet_3_id" {
    value = aws_subnet.private_subnet_3.id
    description = "The Private Subnet 3 of the VPC."
}

output "public_subnet_1_id" {
    value = aws_subnet.public_subnet_1.id
    description = "The Public Subnet 1 of the VPC."
}
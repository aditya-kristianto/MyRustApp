output "id" {
  value = aws_vpc.my_vpc.id
  description = "The ID of the VPC."
}

output "private_subnets" {
  value = [
    aws_subnet.private_ap_southeast_1a.id, 
    aws_subnet.private_ap_southeast_1b.id,
    aws_subnet.private_ap_southeast_1c.id
  ]
  description = "The Private Subnets of the VPC."
}

output "public_subnets" {
  value = [
    aws_subnet.public_ap_southeast_1a.id, 
    aws_subnet.public_ap_southeast_1b.id,
    aws_subnet.public_ap_southeast_1c.id
  ]
  description = "The Public Subnets of the VPC."
}

output "private_ap_southeast_1a" {
    value = aws_subnet.private_ap_southeast_1a.id
    description = "The Private Subnet 1a of the VPC."
}

output "private_ap_southeast_1b" {
    value = aws_subnet.private_ap_southeast_1b.id
    description = "The Private Subnet 1b of the VPC."
}

output "private_ap_southeast_1c" {
    value = aws_subnet.private_ap_southeast_1c.id
    description = "The Private Subnet 1c of the VPC."
}

output "public_ap_southeast_1a" {
    value = aws_subnet.public_ap_southeast_1a.id
    description = "The Public Subnet 1a of the VPC."
}

output "public_ap_southeast_1b" {
    value = aws_subnet.public_ap_southeast_1b.id
    description = "The Public Subnet 1b of the VPC."
}

output "public_ap_southeast_1c" {
    value = aws_subnet.public_ap_southeast_1c.id
    description = "The Public Subnet 1c of the VPC."
}

output "private_ap_southeast_1a_cidr_block" {
    value = aws_subnet.private_ap_southeast_1a.cidr_block
    description = "The Private Subnet 1a CIDR Block of the VPC."
}

output "private_ap_southeast_1b_cidr_block" {
    value = aws_subnet.private_ap_southeast_1b.cidr_block
    description = "The Private Subnet 1b CIDR Block of the VPC."
}

output "private_ap_southeast_1c_cidr_block" {
    value = aws_subnet.private_ap_southeast_1c.cidr_block
    description = "The Private Subnet 1c CIDR Block of the VPC."
}
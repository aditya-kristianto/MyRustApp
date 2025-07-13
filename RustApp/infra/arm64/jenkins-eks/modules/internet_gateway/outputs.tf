output "id" {
  description = "The Internet Gateway ID of the EC2 Instance"
  value       = aws_internet_gateway.my_igw.id
}
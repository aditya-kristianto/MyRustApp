resource "aws_nat_gateway" "nat" {
  allocation_id = var.nat_eip_id
  subnet_id     = var.public_subnet

  tags = {
    Name        = "my-nat-gateway"
    Terraform   = "true"
    Environment = "dev"
  }
}
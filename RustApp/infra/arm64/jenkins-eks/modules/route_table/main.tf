resource "aws_route_table" "public_route_table" {
  vpc_id = var.vpc_id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = var.internet_gateway_id
  }
  
  tags = {
    Name        = "public-subnet-route-table"
    Terraform   = "true"
    Environment = "dev"
  }
}

resource "aws_route_table_association" "public_association" {
  subnet_id      = var.public_subnet
  route_table_id = aws_route_table.public_route_table.id
}
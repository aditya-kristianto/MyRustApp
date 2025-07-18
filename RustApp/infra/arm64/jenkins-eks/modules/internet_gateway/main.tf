# Internet Gateway for NAT
resource "aws_internet_gateway" "my_igw" {
  vpc_id = var.vpc_id
  
  tags = {
    Name        = "my-igw"
    Environment = "dev"
    Terraform   = "true"
  }
}

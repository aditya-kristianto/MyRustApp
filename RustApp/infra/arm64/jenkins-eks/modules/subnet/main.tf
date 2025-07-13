resource "aws_subnet" "public_subnet_1" {
  vpc_id            = var.vpc_id
  cidr_block        = "10.0.101.0/24"
  availability_zone = "${var.aws_region}a"
  map_public_ip_on_launch = true

  tags = {
    Name        = "my-vpc-public-subnet-1"
    Environment = "dev"
    Terraform   = "true"
  }
}

resource "aws_subnet" "private_subnet_1" {
  vpc_id            = var.vpc_id
  cidr_block        = "10.0.1.0/24"
  availability_zone = "${var.aws_region}a"

  tags = {
    Name        = "my-vpc-private-subnet-1"
    Environment = "dev"
    Terraform   = "true"
  }
}

resource "aws_subnet" "public_subnet_2" {
  vpc_id            = var.vpc_id
  cidr_block        = "10.0.102.0/24"
  availability_zone = "${var.aws_region}b"
  map_public_ip_on_launch = true

  tags = {
    Name        = "my-vpc-public-subnet-2"
    Environment = "dev"
    Terraform   = "true"
  }
}

resource "aws_subnet" "private_subnet_2" {
  vpc_id            = var.vpc_id
  cidr_block        = "10.0.2.0/24"
  availability_zone = "${var.aws_region}b"

  tags = {
    Name        = "my-vpc-private-subnet-2"
    Environment = "dev"
    Terraform   = "true"
  }
}

resource "aws_subnet" "public_subnet_3" {
  vpc_id            = var.vpc_id
  cidr_block        = "10.0.103.0/24"
  availability_zone = "${var.aws_region}c"
  map_public_ip_on_launch = true

  tags = {
    Name        = "my-vpc-public-subnet-3"
    Environment = "dev"
    Terraform   = "true"
  }
}

resource "aws_subnet" "private_subnet_3" {
  vpc_id            = var.vpc_id
  cidr_block        = "10.0.3.0/24"
  availability_zone = "${var.aws_region}c"

  tags = {
    Name        = "my-vpc-private-subnet-3"
    Environment = "dev"
    Terraform   = "true"
  }
}
resource "aws_vpc" "my_vpc" {
  cidr_block = "10.0.0.0/16"
  
  tags = {
    Name        = "my-vpc"
    Environment = "dev"
    Terraform   = "true"
  }
}

# Internet Gateway for NAT
resource "aws_internet_gateway" "igw" {
  vpc_id =  aws_vpc.my_vpc.id
  
  tags = {
    Name = "my-igw"
  }
}

resource "aws_subnet" "public_ap_southeast_1a" {
  vpc_id            = aws_vpc.my_vpc.id
  cidr_block        = "10.0.101.0/24"
  availability_zone = "ap-southeast-1a"
  map_public_ip_on_launch = true

  tags = {
    Name                                          = "public-ap-southeast-1a"
    Environment                                   = "dev"
    Terraform                                     = "true"
    "kubernetes.io/role/elb"                      = "1"
    "kubernetes.io/cluster/jenkins-agent-cluster" = "owned"
  }
}

resource "aws_subnet" "public_ap_southeast_1b" {
  vpc_id            = aws_vpc.my_vpc.id
  cidr_block        = "10.0.102.0/24"
  availability_zone = "ap-southeast-1b"
  map_public_ip_on_launch = true

  tags = {
    Name                                          = "public-ap-southeast-1b"
    Environment                                   = "dev"
    Terraform                                     = "true"
    "kubernetes.io/role/elb"                      = "1"
    "kubernetes.io/cluster/jenkins-agent-cluster" = "owned"
  }
}

resource "aws_subnet" "public_ap_southeast_1c" {
  vpc_id            = aws_vpc.my_vpc.id
  cidr_block        = "10.0.103.0/24"
  availability_zone = "ap-southeast-1c"
  map_public_ip_on_launch = true

  tags = {
    Name                                          = "public-ap-southeast-1c"
    Environment                                   = "dev"
    Terraform                                     = "true"
    "kubernetes.io/role/elb"                      = "1"
    "kubernetes.io/cluster/jenkins-agent-cluster" = "owned"
  }
}

resource "aws_subnet" "private_ap_southeast_1a" {
  vpc_id            = aws_vpc.my_vpc.id
  cidr_block        = "10.0.1.0/24"
  availability_zone = "ap-southeast-1a"

  tags = {
    Name                                          = "private-ap-southeast-1a"
    Environment                                   = "dev"
    Terraform                                     = "true"
    "kubernetes.io/role/internal-elb"             = "1"
    "kubernetes.io/cluster/jenkins-agent-cluster" = "owned"
  }
}

resource "aws_subnet" "private_ap_southeast_1b" {
  vpc_id            = aws_vpc.my_vpc.id
  cidr_block        = "10.0.2.0/24"
  availability_zone = "ap-southeast-1b"

  tags = {
    Name                                          = "private-ap-southeast-1b"
    Environment                                   = "dev"
    Terraform                                     = "true"
    "kubernetes.io/role/internal-elb"             = "1"
    "kubernetes.io/cluster/jenkins-agent-cluster" = "owned"
  }
}

resource "aws_subnet" "private_ap_southeast_1c" {
  vpc_id            = aws_vpc.my_vpc.id
  cidr_block        = "10.0.3.0/24"
  availability_zone = "ap-southeast-1c"

  tags = {
    Name                                          = "private-ap-southeast-1c"
    Environment                                   = "dev"
    Terraform                                     = "true"
    "kubernetes.io/role/internal-elb"             = "1"
    "kubernetes.io/cluster/jenkins-agent-cluster" = "owned"
  }
}

resource "aws_route_table_association" "public_ap_southeast_1a_association" {
  subnet_id      = aws_subnet.public_ap_southeast_1a.id
  route_table_id = aws_route_table.public.id
}

resource "aws_route_table_association" "public_ap_southeast_1b_association" {
  subnet_id      = aws_subnet.public_ap_southeast_1b.id
  route_table_id = aws_route_table.public.id
}

resource "aws_route_table_association" "public_ap_southeast_1c_association" {
  subnet_id      = aws_subnet.public_ap_southeast_1c.id
  route_table_id = aws_route_table.public.id
}

resource "aws_route_table" "public" {
  vpc_id = aws_vpc.my_vpc.id
  
  route {
    cidr_block  = "0.0.0.0/0"
    gateway_id  = aws_internet_gateway.igw.id
  }

  tags = {
    Name = "public-route-table"
  }
}

# Private Route Table (routes through NAT)
resource "aws_route_table" "private" {
  vpc_id = aws_vpc.my_vpc.id
  
  route {
    cidr_block      = "0.0.0.0/0"
    nat_gateway_id  = aws_nat_gateway.nat_gw.id
  }

  tags = {
    Name = "private-route-table"
  }
}

resource "aws_route_table_association" "private_ap_southeast_1a_association" {
  subnet_id      = aws_subnet.private_ap_southeast_1a.id
  route_table_id = aws_route_table.private.id
}

resource "aws_route_table_association" "private_ap_southeast_1b_association" {
  subnet_id      = aws_subnet.private_ap_southeast_1b.id
  route_table_id = aws_route_table.private.id
}

resource "aws_route_table_association" "private_ap_southeast_1c_association" {
  subnet_id      = aws_subnet.private_ap_southeast_1c.id
  route_table_id = aws_route_table.private.id
}

resource "aws_eip" "nat_eip" {
  domain = "vpc"
  
  tags = {
    Name = "my-eip"
  }
}

resource "aws_nat_gateway" "nat_gw" {
  allocation_id = aws_eip.nat_eip.id
  subnet_id     = aws_subnet.public_ap_southeast_1a.id
  
  tags = {
    Name = "my-nat"
  }

  depends_on = [aws_internet_gateway.igw]
}



# resource "aws_vpc_endpoint" "ssm_endpoint" {
#   vpc_id            = aws_vpc.my_vpc.id
#   service_name      = "com.amazonaws.ap-southeast-1.ssm"
#   vpc_endpoint_type = "Interface"  # Change to "Interface"
#   subnet_ids        = [
#     aws_subnet.private_ap_southeast_1a.id, 
#     aws_subnet.private_ap_southeast_1b.id, 
#     aws_subnet.private_ap_southeast_1c.id
#   ]
#   security_group_ids = [aws_security_group.vpc_endpoint_sg.id]
  
#   tags = {
#     Name = "SSM Endpoint"
#   }
# }

# resource "aws_vpc_endpoint" "ec2_messages_endpoint" {
#   vpc_id            = aws_vpc.my_vpc.id
#   service_name      = "com.amazonaws.ap-southeast-1.ec2messages"
#   vpc_endpoint_type = "Interface"  # Change to "Interface"
#   subnet_ids        = [
#     aws_subnet.private_ap_southeast_1a.id, 
#     aws_subnet.private_ap_southeast_1b.id, 
#     aws_subnet.private_ap_southeast_1c.id
#   ]
#   security_group_ids = [aws_security_group.vpc_endpoint_sg.id]
  
#   tags = {
#     Name = "EC2 Messages Endpoint"
#   }
# }

# resource "aws_vpc_endpoint" "ssm_messages_endpoint" {
#   vpc_id            = aws_vpc.my_vpc.id
#   service_name      = "com.amazonaws.ap-southeast-1.ssmmessages"
#   vpc_endpoint_type = "Interface"  # Change to "Interface"
#   subnet_ids        = [
#     aws_subnet.private_ap_southeast_1a.id, 
#     aws_subnet.private_ap_southeast_1b.id, 
#     aws_subnet.private_ap_southeast_1c.id
#   ]
#   security_group_ids = [aws_security_group.vpc_endpoint_sg.id]
  
#   tags = {
#     Name = "SSM Messages Endpoint"
#   }
# }

# resource "aws_security_group" "vpc_endpoint_sg" {
#   name        = "vpc_endpoint_security_group"
#   description = "Allow SSM access"
#   vpc_id      = aws_vpc.my_vpc.id  # Replace with your VPC ID

#   ingress {
#     description      = "Allow HTTPS traffic"
#     from_port        = 443
#     to_port          = 443
#     protocol         = "tcp"
#     cidr_blocks      = ["0.0.0.0/0"]  # Replace with your VPC CIDR range
#     ipv6_cidr_blocks = ["::/0"]
#   }
  
#   tags = {
#     Name        = "vpc_endpoint_security_group"
#     Environment = "dev"
#     Terraform   = "true"
#   }
# }
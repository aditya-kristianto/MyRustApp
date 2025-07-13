resource "aws_security_group" "ec2_bastion_host_sg" {
  name        = "ec2_ssm_security_group"
  description = "Allow SSM access"
  vpc_id      = var.vpc_id
  
  ingress {
    description      = "Allow HTTPS traffic"
    from_port        = 22
    to_port          = 22
    protocol         = "tcp"
    cidr_blocks      = ["0.0.0.0/0"]  # Replace with your VPC CIDR range
    ipv6_cidr_blocks = ["::/0"]
  }
  
  egress {
    description      = "Allow HTTPS traffic"
    from_port        = 22
    to_port          = 22
    protocol         = "tcp"
    cidr_blocks      = var.bastion_host_cidr_blocks
  }

  egress {
    description      = "Allow HTTPS traffic"
    from_port        = 443
    to_port          = 443
    protocol         = "tcp"
    cidr_blocks      = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
  }
  
  tags = {
    Name        = "ec2_ssm_security_group"
    Environment = "dev"
    Terraform   = "true"
  }
}

resource "aws_security_group" "ec2_portainer_sg" {
  name        = "ec2_portainer_security_group"
  description = "Allow SSH access"
  vpc_id      = var.vpc_id
  
  ingress {
    description = "Allow traffic from the Bastion Host security group"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = var.portainer_cidr_blocks
  }
  
  ingress {
    description = "Allows ICMP from the Bastion Host security group"
    from_port   = -1
    to_port     = -1
    protocol    = "icmp"
    cidr_blocks = var.portainer_cidr_blocks
  }
  
  tags = {
    Name        = "ec2_portainer_security_group"
    Environment = "dev"
    Terraform   = "true"
  }
}
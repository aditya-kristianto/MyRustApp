terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "5.68.0"  # Adjust to the latest stable version
    }
  }
}

# ---- Get the current AWS account ID ----
data "aws_caller_identity" "current" {}

provider "aws" {
  region = var.region
}

resource "aws_security_group" "postgres_sg" {
  vpc_id = var.vpc_id  # Ensure the security group is in the correct VPC

  name        = "postgres-sg"
  description = "Allow PostgreSQL access from specific IPs"

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["180.244.154.21/32"]  # Whitelist the specific IPv4 address
  }
  
  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    ipv6_cidr_blocks = ["2001:448a:2061:44ba::/128"]  # Whitelist the specific IPv6 address
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"  # Allow all outbound traffic
    cidr_blocks = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]  # Allow all outbound traffic for IPv6
  }
  
  tags = var.sg_tags
}

resource "aws_instance" "postgres_instance" {
  ami           = "ami-094ebf7c110197581"  # Use a suitable Amazon Linux 2 AMI
  instance_type = "t4g.micro"  # Change as needed
  
  subnet_id     = var.subnet_id  # Use the subnet created in the VPC
  vpc_security_group_ids = [aws_security_group.postgres_sg.id]  # Attach the security group
  iam_instance_profile   = aws_iam_instance_profile.ssm_instance_profile.name
  
  # Adding elastic IP address here
  associate_public_ip_address = false

  # User data script to install Docker and run PostgreSQL
  user_data = <<-EOF
              #!/bin/bash
              yum update -y
              yum install -y docker
              systemctl enable docker  # Enable Docker to start on boot
              service docker start
              
              # Create a custom bridge network
              docker network create \
                --subnet=172.18.0.0/16 my_network
              
              # Run PostgreSQL
              docker run --name postgres \
                --network my_network \
                --ip 172.18.0.3 \
                --restart always \
                -e POSTGRES_PASSWORD=mysecretpassword \
                -d public.ecr.aws/docker/library/postgres:12.20-alpine3.20
              
              # Run pgAdmin
              docker run --name pgadmin \
                --network my_network \
                --ip 172.18.0.2 \
                --restart always \
                -e "PGADMIN_DEFAULT_EMAIL=kristianto.aditya@gmail.com" \
                -e "PGADMIN_DEFAULT_PASSWORD=mysecretpassword" \
                -p 80:80 \
                -d dpage/pgadmin4
              EOF

  tags = var.ec2_tags
}

resource "aws_route53_record" "pgadmin_record" {
  zone_id  = "Z0148722AZOXRNZ0DFO2"  # Use your existing hosted zone ID
  name     = "pgadmin"
  type     = "A"
  ttl      = 300

  # Set the value to the public IP of the EC2 instance
  records = [aws_eip.elastic_ip.public_ip]
}

# Allocate an Elastic IP
resource "aws_eip" "elastic_ip" {
  vpc = true  # Make sure it's in the same VPC
}

# Associate the Elastic IP with the EC2 instance
resource "aws_eip_association" "eip_assoc" {
  instance_id = aws_instance.postgres_instance.id
  allocation_id = aws_eip.elastic_ip.id
}

# VPC Endpoint for SSM
resource "aws_vpc_endpoint" "ssm" {
  vpc_id            = var.vpc_id
  service_name      = "com.amazonaws.${var.region}.ssm"
  vpc_endpoint_type = "Interface"
  subnet_ids        = [var.subnet_id]
  security_group_ids = [aws_security_group.postgres_sg.id]
}

# VPC Endpoint for EC2 messages (required for SSM)
resource "aws_vpc_endpoint" "ec2_messages" {
  vpc_id            = var.vpc_id
  service_name      = "com.amazonaws.${var.region}.ec2messages"
  vpc_endpoint_type = "Interface"
  subnet_ids        = [var.subnet_id]
  security_group_ids = [aws_security_group.postgres_sg.id]
}

# VPC Endpoint for SSM messages (required for SSM)
resource "aws_vpc_endpoint" "ssm_messages" {
  vpc_id            = var.vpc_id
  service_name      = "com.amazonaws.${var.region}.ssmmessages"
  vpc_endpoint_type = "Interface"
  subnet_ids        = [var.subnet_id]
  security_group_ids = [aws_security_group.postgres_sg.id]
}

# IAM Role for SSM
resource "aws_iam_role" "ssm_role" {
  name = "ssm-role"
  
  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [{
      Action = "sts:AssumeRole",
      Effect = "Allow",
      Principal = {
        Service = "ec2.amazonaws.com"
      }
    }]
  })
}

# Attach required policies for SSM Session Manager
resource "aws_iam_role_policy_attachment" "ssm_managed_policy" {
  role       = aws_iam_role.ssm_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

# Instance Profile for attaching IAM Role to EC2 instance
resource "aws_iam_instance_profile" "ssm_instance_profile" {
  name = "ssm-instance-profile"
  role = aws_iam_role.ssm_role.name
}

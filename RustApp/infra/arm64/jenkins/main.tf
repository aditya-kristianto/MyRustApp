# Provider Configuration
provider "aws" {
  region = var.aws_region
}

module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "5.13.0"

  name = "jenkins-vpc"
  cidr = "10.0.0.0/16"

  azs             = ["${var.aws_region}a", "${var.aws_region}b", "${var.aws_region}c"]
  private_subnets = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  public_subnets  = ["10.0.4.0/24", "10.0.5.0/24", "10.0.6.0/24"]

  enable_nat_gateway = true
  single_nat_gateway = true
  enable_dns_hostnames = true
  
  public_subnet_tags = {
    "kubernetes.io/role/elb" = 1
  }

  private_subnet_tags = {
    "kubernetes.io/role/internal-elb" = 1
  }

  tags = {
    Terraform   = "true"
    Environment = "dev"
  }
}

# Key Pair
resource "aws_key_pair" "jenkins_key" {
  key_name   = "jenkins-key"
  public_key = file("~/.ssh/id_rsa.pub") # Path to your public SSH key
}

# Security Group to allow traffic
resource "aws_security_group" "jenkins_sg" {
  vpc_id = var.vpc_id  # Ensure the security group is in the correct VPC


  name        = "jenkins-sg"
  description = "Allow HTTP, HTTPS, and SSH traffic"

#   ingress {
#     from_port   = 22
#     to_port     = 22
#     protocol    = "tcp"
#     cidr_blocks = ["0.0.0.0/0"] # Allow SSH from anywhere (adjust to your IP)
#   }

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"] # Allow Jenkins web interface (port 8080)
  }
  
  ingress {
    from_port   = 50000
    to_port     = 60000
    protocol    = "tcp"
    cidr_blocks = ["${aws_eip.elastic_ip.public_ip}/32"] # Allow Jenkins web interface (port 5000 to 6000)
  }
  
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"] # Allow all outbound traffic
  }
}

# EC2 Instance to run Jenkins
resource "aws_instance" "jenkins_instance" {
  ami           = "ami-094ebf7c110197581"  # Use a suitable Amazon Linux 2 AMI
  instance_type = "t4g.medium"  # Change as needed
  
  subnet_id     = var.subnet_id  # Use the subnet created in the VPC
  vpc_security_group_ids = [aws_security_group.jenkins_sg.id]  # Attach the security group
  iam_instance_profile   = data.aws_iam_instance_profile.ssm_instance_profile.name

#   security_groups = [aws_security_group.jenkins_sg.name]

  # Adding elastic IP address here
  associate_public_ip_address = false

  # Install Docker and run Jenkins in a Docker container
  user_data = <<-EOF
              #!/bin/bash
              # Install updates and docker
              yum update -y
              yum install -y docker git java
              systemctl start docker
              systemctl enable docker  # Enable Docker to start on boot

              # Pull Jenkins Docker image and run it
              docker run --name jenkins \
                -p 80:8080 \
                -p 50000:50000 \
                --restart unless-stopped \
                --cpus="1.5" \
                --memory="3g" \
                --memory-swap="3g" \
                -d jenkins/jenkins:lts
                
              mount -o remount,size=5G /tmp/
              
              ssh-keygen -t rsa -b 4096 -C "kristianto.aditya@gmail.com"
              
              # Create the SSH config file
              cat <<EOT >> /home/ssm-user/.ssh/config
              Host codecommit
                  HostName git-codecommit.ap-southeast-1.amazonaws.com
                  User <username> # Replace with your IAM username
                  IdentityFile /home/ssm-user/.ssh/id_rsa
              EOT
              
              chmod 600 /home/ssm-user/.ssh/id_rsa
              chmod 644 /home/ssm-user/.ssh/id_rsa.pub
              chmod 600 /home/ssm-user/.ssh/config
              EOF

  # Adding a 50GB EBS volume
  root_block_device {
    volume_size = 50                # Set the volume size to 50GB
    volume_type = "gp3"             # General Purpose SSD (gp3 or gp2)
    delete_on_termination = true    # Delete the volume when the instance is terminated
  }
  
  tags = {
    Name = "Jenkins Instance"
  }
}

resource "aws_route53_record" "jenkins_record" {
  zone_id  = "Z0148722AZOXRNZ0DFO2"  # Use your existing hosted zone ID
  name     = "jenkins"
  type     = "A"
  ttl      = 300

  # Set the value to the public IP of the EC2 instance
  records = [aws_eip.elastic_ip.public_ip]
}

# Allocate an Elastic IP
resource "aws_eip" "elastic_ip" {
  domain = "vpc"  # Make sure it's in the same VPC
}

# Associate the Elastic IP with the EC2 instance
resource "aws_eip_association" "eip_assoc" {
  instance_id = aws_instance.jenkins_instance.id
  allocation_id = aws_eip.elastic_ip.id
}

# VPC Endpoint for SSM
resource "aws_vpc_endpoint" "ssm" {
  vpc_id            = module.vpc.vpc_id
  service_name      = "com.amazonaws.${var.aws_region}.ssm"
  vpc_endpoint_type = "Interface"
  subnet_ids        = [var.subnet_id]
  security_group_ids = [aws_security_group.jenkins_sg.id]
}

# VPC Endpoint for EC2 messages (required for SSM)
resource "aws_vpc_endpoint" "ec2_messages" {
  vpc_id            = module.vpc.vpc_id
  service_name      = "com.amazonaws.${var.aws_region}.ec2messages"
  vpc_endpoint_type = "Interface"
  subnet_ids        = [var.subnet_id]
  security_group_ids = [aws_security_group.jenkins_sg.id]
}

# VPC Endpoint for SSM messages (required for SSM)
resource "aws_vpc_endpoint" "ssm_messages" {
  vpc_id            = module.vpc.vpc_id
  service_name      = "com.amazonaws.${var.aws_region}.ssmmessages"
  vpc_endpoint_type = "Interface"
  subnet_ids        = [var.subnet_id]
  security_group_ids = [aws_security_group.jenkins_sg.id]
}

# Fetch the existing IAM role
data "aws_iam_role" "existing_ssm_role" {
  name = "ssm-role"  # Replace with the actual role name
}

# Attach required policies for SSM Session Manager
resource "aws_iam_role_policy_attachment" "ssm_managed_policy" {
  role       = data.aws_iam_role.existing_ssm_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

# Instance Profile for attaching IAM Role to EC2 instance
data "aws_iam_instance_profile" "ssm_instance_profile" {
  name = "ssm-instance-profile"
}

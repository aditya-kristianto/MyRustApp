# EC2 Instance for Portainer
# ==============================================================================
resource "aws_instance" "portainer" {
  ami           = "ami-05c718cb2fa58687f"
  instance_type = "t4g.nano"
  key_name      = var.key_name

  subnet_id              = var.private_subnet_id
  vpc_security_group_ids = [aws_security_group.ec2_portainer_sg.id]
  iam_instance_profile   = var.iam_instance_profile_name

  # Adding elastic IP address here
  associate_public_ip_address = false

  user_data = <<-EOF
              #!/bin/bash
              sudo yum update -y
              sudo yum install -y amazon-ssm-agent docker
              systemctl enable amazon-ssm-agent
              systemctl start amazon-ssm-agent
              systemctl start docker
              systemctl enable docker  # Enable Docker to start on boot

              # Pull Jenkins Docker image and run it
              docker volume create portainer_data
              docker run -d --name portainer \
                -p 80:8000 \
                -p 443:9443 \
                --restart unless-stopped \
                -v /var/run/docker.sock:/var/run/docker.sock \
                -v portainer_data:/data \
                portainer/portainer-ce:2.24.0-alpine
              docker restart portainer
              EOF
              
  # Adding a 50GB EBS volume
  root_block_device {
    volume_size = 10                # Set the volume size to 10GB
    volume_type = "gp3"             # General Purpose SSD (gp3 or gp2)
    delete_on_termination = true    # Delete the volume when the instance is terminated
  }
  
  tags = {
    Environment = "dev"
    Name        = "portainer"
    Terraform   = "true"
  }
}
# ==============================================================================

resource "aws_security_group" "ec2_portainer_sg" {
  name        = "ec2_portainer_security_group"
  description = "Allow SSH access"
  vpc_id      = var.vpc_id
  
  ingress {
    description = "Allow traffic from the Bastion Host security group"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = var.cidr_blocks
  }
  
  ingress {
    description = "Allows ICMP from the Bastion Host security group"
    from_port   = -1
    to_port     = -1
    protocol    = "icmp"
    cidr_blocks = var.cidr_blocks
  }
  
  ingress {
    description = "HTTP"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  ingress {
    description = "HTTPS"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  egress {
    description = "Allow all outbound traffic"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  tags = {
    Name        = "ec2_portainer_security_group"
    Environment = "dev"
    Terraform   = "true"
  }
}
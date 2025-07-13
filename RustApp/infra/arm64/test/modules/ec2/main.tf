# EC2 Instance for Bastion Host
# ==============================================================================
resource "aws_instance" "bastion_host" {
  ami                    = "ami-015ce1b8f0618ad32"  # Replace with a suitable Amazon Linux 2 AMI ID
  instance_type          = "t4g.nano"
  subnet_id              = var.public_subnet_id
  vpc_security_group_ids = var.bastion_host_security_group_ids
  key_name               = var.key_name
  iam_instance_profile   = var.iam_instance_profile_name

  tags = {
    Name = "bastion-host"
    Environment = "dev"
    Terraform   = "true"
  }
}
# ==============================================================================

# EC2 Instance for Portainer
# ==============================================================================
resource "aws_instance" "portainer" {
  ami           = "ami-05c718cb2fa58687f"
  instance_type = "t4g.nano"
  key_name      = var.key_name

  subnet_id              = var.private_subnet_id
  vpc_security_group_ids = var.portainer_security_group_ids
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
                --restart=always \
                -v /var/run/docker.sock:/var/run/docker.sock \
                -v portainer_data:/data \
                portainer/portainer-ce:2.21.4
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

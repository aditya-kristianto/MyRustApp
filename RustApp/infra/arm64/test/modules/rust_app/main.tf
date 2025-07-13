# EC2 Instance for rust_app
# ==============================================================================
resource "aws_instance" "rust_app" {
  ami           = "ami-05c718cb2fa58687f"
  instance_type = "t4g.nano"
  key_name      = var.key_name

  subnet_id              = var.private_subnet_id
  vpc_security_group_ids = [aws_security_group.ec2_rust_app_sg.id]
  iam_instance_profile   = var.iam_instance_profile_name

  # Adding elastic IP address here
  associate_public_ip_address = false

  user_data = <<-EOF
              #!/bin/bash
              echo "$(date '+%Y-%m-%d %H:%M:%S') - Starting user data execution" >> /var/log/user-data.log
    
              sudo yum update -y
              sudo yum install -y amazon-ssm-agent docker htop postgresql15
              systemctl enable amazon-ssm-agent
              systemctl start amazon-ssm-agent
              systemctl start docker
              systemctl enable docker  # Enable Docker to start on boot
              
              aws configure set aws_access_key_id AKIAW4KRWEJLMXNKNYVW --profile terraform && \
              aws configure set aws_secret_access_key 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 --profile terraform && \
              aws configure set region ap-southeast-1 --profile terraform && \
              aws configure set output json --profile terraform
              
              export AWS_PROFILE=terraform

              aws ecr get-login-password --region ap-southeast-1 | docker login --username AWS --password-stdin 473154593366.dkr.ecr.ap-southeast-1.amazonaws.com

              echo "$(date '+%Y-%m-%d %H:%M:%S') - Create Docker Network" >> /var/log/user-data.log
              docker network create \
                --subnet=172.18.0.0/16 \
                --gateway=172.18.0.254 \
                my_network
                
              echo "$(date '+%Y-%m-%d %H:%M:%S') - Run Container rust_web_app" >> /var/log/user-data.log
              # Run Rust App Web Image
              docker run -d --name rust_web_app \
                -p 80:80 \
                --network my_network \
                --ip 172.18.0.1 \
                --restart unless-stopped \
                -e APP_WEB_HTTP_PORT="80" \
                473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/rust-web:1.0.11-release
                
              echo "$(date '+%Y-%m-%d %H:%M:%S') - Check Postgres Connection" >> /var/log/user-data.log
              while ! pg_isready -h ${var.postgres_private_ip} -p 5432 -U aditya.kristianto -d rust_app; do
                sleep 5
              done
                
              echo "$(date '+%Y-%m-%d %H:%M:%S') - Run Container rust_auth_app" >> /var/log/user-data.log
              docker run -d --name rust_auth_app \
                -p 81:80 \
                --network my_network \
                --ip 172.18.0.2 \
                --restart unless-stopped \
                -e APP_WEB_HTTP_PORT="80" \
                -e POSTGRES_DATABASE="rust_app" \
                -e POSTGRES_HOST="${var.postgres_private_ip}" \
                -e POSTGRES_PORT="5432" \
                -e POSTGRES_USERNAME="aditya.kristianto" \
                -e POSTGRES_PASSWORD="my secret password" \
                -e POSTGRES_URI="postgres://aditya.kristianto:my secret password@${var.postgres_private_ip}:5432/rust_app" \
                473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/rust-auth:1.0.8-release
                
              echo "$(date '+%Y-%m-%d %H:%M:%S') - Run Container rust_oauth_app" >> /var/log/user-data.log
              docker run -d --name rust_oauth_app \
                -p 82:80 \
                --network my_network \
                --ip 172.18.0.3 \
                --restart unless-stopped \
                -e APP_WEB_HTTP_PORT="80" \
                -e POSTGRES_DATABASE="rust_app" \
                -e POSTGRES_HOST="${var.postgres_private_ip}" \
                -e POSTGRES_PORT="5432" \
                -e POSTGRES_USERNAME="aditya.kristianto" \
                -e POSTGRES_PASSWORD="my secret password" \
                -e POSTGRES_URI="postgres://aditya.kristianto:my secret password@${var.postgres_private_ip}:5432/rust_app" \
                473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/rust-oauth:1.0.8-release
              
              echo "$(date '+%Y-%m-%d %H:%M:%S') - Run Container rust_stock_app" >> /var/log/user-data.log
              docker run -d --name rust_stock_app \
                -p 83:80 \
                --network my_network \
                --ip 172.18.0.4 \
                --restart unless-stopped \
                -e APP_WEB_HTTP_PORT="80" \
                -e POSTGRES_DATABASE="rust_app" \
                -e POSTGRES_HOST="${var.postgres_private_ip}" \
                -e POSTGRES_PORT="5432" \
                -e POSTGRES_USERNAME="aditya.kristianto" \
                -e POSTGRES_PASSWORD="my secret password" \
                -e POSTGRES_URI="postgres://aditya.kristianto:my secret password@${var.postgres_private_ip}:5432/rust_app" \
                473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/rust-stock:1.0.17-release
               
              echo "$(date '+%Y-%m-%d %H:%M:%S') - Run Container rust_uuid_app" >> /var/log/user-data.log
              docker run -d --name rust_uuid_app \
                -p 84:80 \
                --network my_network \
                --ip 172.18.0.5 \
                --restart unless-stopped \
                -e APP_WEB_HTTP_PORT="80" \
                -e POSTGRES_DATABASE="rust_app" \
                -e POSTGRES_HOST="${var.postgres_private_ip}" \
                -e POSTGRES_PORT="5432" \
                -e POSTGRES_USERNAME="aditya.kristianto" \
                -e POSTGRES_PASSWORD="my secret password" \
                -e POSTGRES_URI="postgres://aditya.kristianto:my secret password@${var.postgres_private_ip}:5432/rust_app" \
                473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/rust-uuid:1.0.8-release
                
              echo "$(date '+%Y-%m-%d %H:%M:%S') - Run Container rust_migration_app" >> /var/log/user-data.log
              docker run -d --name rust_migration_app \
                --network my_network \
                --ip 172.18.0.6 \
                --restart no \
                -e POSTGRES_URI="postgres://aditya.kristianto:my secret password@${var.postgres_private_ip}:5432/rust_app" \
                473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/rust-migration:1.0.7
                
              # Wait for the Rust Migration App container to exited
              echo "Waiting for Rust Migration App to be exited..."
              until [ "$(docker inspect --format='{{.State.Status}}' rust_migration_app)" == "exited" ]; do
                echo "$(date '+%Y-%m-%d %H:%M:%S') - Sleep 1 second" >> /var/log/user-data.log
                sleep 1
              done
              
              # docker stop rust_migration_app
              # docker rm rust_migration_app
              docker logs rust_migration_app >> /var/log/user-data.log 2>&1
              
              echo "$(date '+%Y-%m-%d %H:%M:%S') - User data execution completed" >> /var/log/user-data.log
              EOF
              
  # Adding a 50GB EBS volume
  root_block_device {
    volume_size = 10                # Set the volume size to 10GB
    volume_type = "gp3"             # General Purpose SSD (gp3 or gp2)
    delete_on_termination = true    # Delete the volume when the instance is terminated
  }
  
  tags = {
    Environment = "dev"
    Name        = "rust_app"
    Terraform   = "true"
  }
}
# ==============================================================================

resource "aws_security_group" "ec2_rust_app_sg" {
  name        = "ec2_rust_app_security_group"
  description = "Allow access"
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
    cidr_blocks = ["10.0.0.0/16"]
  }
  
  ingress {
    description = "HTTP"
    from_port   = 81
    to_port     = 81
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
  
  ingress {
    description = "HTTP"
    from_port   = 82
    to_port     = 82
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
  
  ingress {
    description = "HTTP"
    from_port   = 83
    to_port     = 83
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
  
  ingress {
    description = "HTTP"
    from_port   = 84
    to_port     = 84
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
  
  egress {
    description = "Allow all outbound traffic"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  egress {
    description = "Allow outbound traffic to Postgres Instance"
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["${var.postgres_private_ip}/32"]
  }
  
  tags = {
    Name        = "ec2_rust_app_security_group"
    Environment = "dev"
    Terraform   = "true"
  }
}
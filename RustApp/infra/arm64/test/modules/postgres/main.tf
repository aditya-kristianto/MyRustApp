# EC2 Instance for Postgres
# ==============================================================================
resource "aws_instance" "postgres" {
  ami           = "ami-05c718cb2fa58687f"
  instance_type = "t4g.micro"
  key_name      = var.key_name

  subnet_id              = var.private_subnet_id
  vpc_security_group_ids = [aws_security_group.ec2_postgres_sg.id]
  iam_instance_profile   = var.iam_instance_profile_name

  # Adding elastic IP address here
  associate_public_ip_address = false

  user_data = <<-EOF
              #!/bin/bash
              echo "$(date '+%Y-%m-%d %H:%M:%S') - Starting user data execution" >> /var/log/user-data.log
              
              aws s3 cp "s3://rust-app-bucket/databases/rust_app/backups/rust_app_backup_2024_12_14.sql" "$(pwd)/backup.sql"
    
              sudo yum update -y
              sudo yum install -y amazon-ssm-agent docker htop postgresql15
              systemctl enable amazon-ssm-agent
              systemctl start amazon-ssm-agent
              systemctl start docker
              systemctl enable docker  # Enable Docker to start on boot

              # Create a custom bridge network
              docker network create \
                --subnet=172.18.0.0/16 \
                --gateway=172.18.0.254 \
                my_network
                
              # Run PostgreSQL
              docker volume create postgres_data
              docker run -d --name postgres \
                -p 5432:5432 \
                --network my_network \
                --ip 172.18.0.1 \
                --restart unless-stopped \
                -e 'POSTGRES_USER=aditya.kristianto' \
                -e 'POSTGRES_PASSWORD=my secret password' \
                -e 'POSTGRES_DB=postgres' \
	              -e 'PGDATA=/var/lib/postgresql/data/pgdata' \
	              -v postgres_data:/var/lib/postgresql/data \
	              -v /path/to/init-scripts:/docker-entrypoint-initdb.d \
	              -v $(pwd):/tmp \
	              --health-cmd="pg_isready -U aditya.kristianto -d postgres" \
                --health-interval=10s \
                --health-timeout=5s \
                --health-retries=5 \
                postgres:17.2-alpine3.21
                
              # Define the file path where the JSON will be created
              output_file="servers.json"
              
              # Define the JSON content
              json_content='{
                "Servers": {
                  "1": {
                    "Name": "My Postgres Server",
                    "Group": "Servers",
                    "Host": "172.18.0.1",
                    "Port": 5432,
                    "Username": "aditya.kristianto",
                    "Password": "my secret password",
                    "MaintenanceDB": "postgres",
                    "SavePassword": true
                  }
                }
              }'
              
              # Write the JSON content to the file
              echo "$json_content" > "$output_file"
              
              # Confirm creation
              echo "JSON file created at $output_file"
                
              # Run pgAdmin
              docker run -d --name pgadmin \
                -p 80:80 \
                -p 443:443 \
                --network my_network \
                --ip 172.18.0.2 \
                --restart unless-stopped \
                -e 'PGADMIN_DEFAULT_EMAIL=kristianto.aditya@gmail.com' \
                -e 'PGADMIN_DEFAULT_PASSWORD=my secret password' \
                -v $(pwd)/servers.json:/pgadmin4/servers.json \
                dpage/pgadmin4:8.14.0
                
              # Wait for the PostgreSQL container to start
              echo "Waiting for PostgreSQL to be healthy..."
              until [ "$(docker inspect --format='{{.State.Health.Status}}' postgres)" == "healthy" ]; do
                echo "$(date '+%Y-%m-%d %H:%M:%S') - Sleep 1 second" >> /var/log/user-data.log
                sleep 1
              done
              
              # Run your database commands
              docker exec -i postgres \
                psql -U aditya.kristianto -d postgres -c "CREATE DATABASE rust_app;" \
                >> /var/log/user-data.log 2>&1
                
              docker exec -i postgres \
                psql -U aditya.kristianto -d rust_app < $(pwd)/backup.sql \
                >> /var/log/user-data.log 2>&1
              
              # docker exec -i postgres pg_dump -U aditya.kristianto -d rust_app -Fc -f /tmp/backup.dump
              # docker exec -i postgres pg_restore  -U aditya.kristianto -d rust_app /tmp/backup.dump
              
              # Backup the data from database
              # cd ~
              # docker exec -i postgres pg_dump -U aditya.kristianto -d rust_app > $(pwd)/backup.sql
              # aws s3 cp $(pwd)/backup.sql s3://rust-app-bucket/databases/rust_app/backups/rust_app_backup_2024_12_14.sql
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
    Name        = "postgres"
    Terraform   = "true"
  }
}
# ==============================================================================

resource "aws_security_group" "ec2_postgres_sg" {
  name        = "ec2_postgres_security_group"
  description = "Allow access"
  vpc_id      = var.vpc_id
  
  ingress {
    description = "Allows ICMP from the Internal VPC"
    from_port   = -1
    to_port     = -1
    protocol    = "icmp"
    cidr_blocks = ["10.0.0.0/16"]
  }
  
  ingress {
    description = "Allow traffic from Internal VPC"
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
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
    Name        = "ec2_postgres_security_group"
    Environment = "dev"
    Terraform   = "true"
  }
}
# EC2 Instance for Jenkins Master
# ==============================================================================
resource "aws_instance" "jenkins_master" {
  ami           = "ami-05c718cb2fa58687f"
  instance_type = "t4g.small"
  key_name      = var.key_name

  subnet_id              = var.private_subnet_id
  vpc_security_group_ids = [aws_security_group.jenkins_master_sg.id]
  iam_instance_profile   = var.iam_instance_profile_name

  # Adding elastic IP address here
  associate_public_ip_address = false

  user_data = <<-EOF
              #!/bin/bash
              sudo yum update -y
              sudo yum install -y amazon-ssm-agent docker git java
              systemctl enable amazon-ssm-agent
              systemctl start amazon-ssm-agent
              systemctl start docker
              systemctl enable docker  # Enable Docker to start on boot
              
              mkdir -p ~/certs
              
              docker volume create jenkins_home

              # Place certificate in the correct location
              # echo "${local_file.jenkins_master_private_key_file.content}" | base64 --decode > ~/certs/jenkins_master_key.pem
              # echo "${local_file.certificate_file.content}" | base64 --decode > ~/certs/certificate.pem
              
              CERT_CONTENT=$(terraform output -raw certificate_content)
              KEY_CONTENT=$(terraform output -raw jenkins_master_private_key_content)
              
              echo "$CERT_CONTENT" | base64 --decode > ~/certs/certificate.pem
              echo "KEY_CONTENT" | base64 --decode > ~/certs/jenkins_master_key.pem
              
              openssl pkcs12 -inkey ~/certs/jenkins_master_key.pem -in ~/certs/certificate.pem -export -out ~/certs/certificate.p12 -passout pass:my_secure_password
              
              keytool -importkeystore -srckeystore  ~/certs/certificate.p12 -srcstoretype pkcs12 -srcstorepass my_secure_password -destkeystore  /var/lib/docker/volumes/jenkins_home/_data/jenkins_master_key.jks -deststoretype JKS -deststorepass my_secure_password
              
              # Pull Jenkins Docker image and run it
              docker run --name jenkins \
                -p 80:8080 \
                -p 443:8443 \
                -p 50000:50000 \
                --restart unless-stopped \
                --cpus="1" \
                --memory="1.5g" \
                --memory-swap="1.5g" \
                -v jenkins_home:/data \
                -e JENKINS_OPTS="--httpPort=-1 --httpsPort=8443 --httpsKeyStore=jenkins_master_key.jks --httpsKeyStorePassword=my_secure_password" \
                -d jenkins/jenkins:2.486-slim-jdk21
                
              mount -o remount,size=5G /tmp/
              
              mkdir -p jenkins_certs
              # openssl req -newkey rsa:2048 -nodes -keyout jenkins_certs/jenkins.key -x509 -days 365 -out jenkins_certs/jenkins.crt -subj "/CN=localhost"

              
              
              # ssh-keygen -t rsa -b 4096 -C "kristianto.aditya@gmail.com"
              
              # # Install Jenkins plugins
              # cat <<EOT > /usr/share/jenkins/ref/plugins.txt
              # kubernetes:1.35.3
              # credentials-binding:1.27
              # workflow-aggregator:2.6
              # git:4.13.0
              # pipeline-stage-view:2.23
              # EOT
              # # sudo wget -O /usr/share/jenkins/ref/plugins.txt https://raw.githubusercontent.com/your-repo/plugins.txt
              # sudo /usr/bin/install-plugins.sh < /usr/share/jenkins/ref/plugins.txt
              EOF
              
  # Adding a 50GB EBS volume
  root_block_device {
    volume_size = 50                # Set the volume size to 50GB
    volume_type = "gp3"             # General Purpose SSD (gp3 or gp2)
    delete_on_termination = true    # Delete the volume when the instance is terminated
  }

  # provisioner "local-exec" {
  #   command = "echo 'Jenkins Master EC2 instance (${aws_instance.jenkins_master.public_ip}) is up and running.'"
  # }
  
  depends_on = [
    local_file.jenkins_master_private_key_file,
    local_file.jenkins_master_public_key_file,
    local_file.certificate_file
  ]
  
  tags = {
    Environment = "dev"
    Name        = "jenkins-master"
    Terraform   = "true"
  }
}
# ==============================================================================

# Security Group for Jenkins Master EC2
# ==============================================================================
resource "aws_security_group" "jenkins_master_sg" {
  name        = "jenkins-master-sg"
  description = "Security group for Jenkins Master EC2 instance"
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
  
  ingress {
    from_port   = 50000
    to_port     = 50000
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  ingress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
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
    Name        = "jenkins-master-sg"
    Environment = "dev"
    Terraform   = "true"
  }
}
# ==============================================================================

resource "tls_private_key" "jenkins_master_key" {
  algorithm = "RSA"
  rsa_bits  = 2048
}

resource "aws_key_pair" "jenkins_master_key_pair" {
  key_name   = "my-jenkins-master-key-pair"
  public_key = tls_private_key.jenkins_master_key.public_key_openssh
}

resource "local_file" "jenkins_master_private_key_file" {
  content  = tls_private_key.jenkins_master_key.private_key_pem
  filename = "${path.module}/jenkins_master_key.pem"
}

resource "local_file" "jenkins_master_public_key_file" {
  content  = tls_private_key.jenkins_master_key.public_key_openssh
  filename = "${path.module}/jenkins_master_key.pub"
}

resource "tls_self_signed_cert" "jenkins_master" {
  private_key_pem = tls_private_key.jenkins_master_key.private_key_pem

  validity_period_hours = 8760  # 1 year
  allowed_uses = [
    "key_encipherment",
    "digital_signature",
    "server_auth",
  ]

  subject {
    common_name  = "jenkins.aditya-kristianto.com"
    organization = "Aditya Kristianto"
    country      = "ID"
  }
}

resource "local_file" "certificate_file" {
  content  = tls_self_signed_cert.jenkins_master.cert_pem
  filename = "${path.module}/certificate.pem"
}
# Security Group for Jenkins Agent EC2
# ==============================================================================
resource "aws_security_group" "jenkins_agent_sg" {
  name        = var.jenkins_agent_ec2_security_group_name
  description = "Security group for Jenkins Agent EC2 instance"
  vpc_id      = var.vpc_id

  ingress {
    description = "HTTP"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 10250
    to_port     = 10250
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  ingress {
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    cidr_blocks      = ["0.0.0.0/0"]
  }

  egress {
    description = "Allow all outbound traffic"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name        = "jenkins-agent-sg"
    Environment = "dev"
    Terraform   = "true"
  }
}
# ==============================================================================

# Security Group for Jenkins Master EC2
# ==============================================================================
resource "aws_security_group" "jenkins_master_sg" {
  name        = var.jenkins_master_ec2_security_group_name
  description = "Security group for Jenkins Master EC2 instance"
  vpc_id      = var.vpc_id

  ingress {
    description = "HTTP"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  ingress {
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
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    cidr_blocks      = ["0.0.0.0/0"]
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

# ==============================================================================
resource "aws_security_group" "portainer_sg" {
  name        = var.portainer_ec2_security_group_name
  description = "Security group for Portainer EC2 instance"
  vpc_id      = var.vpc_id

  ingress {
    description = "HTTP"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  ingress {
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    cidr_blocks      = ["0.0.0.0/0"]
  }
  
  ingress {
    from_port   = 50000
    to_port     = 50000
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
    Name        = "portainer-sg"
    Environment = "dev"
    Terraform   = "true"
  }
}
# ==============================================================================

# Ingress Rule for VPC Endpoints Security Group
# ==============================================================================
# resource "aws_security_group_rule" "allow_ssm_ingress" {
#   type              = "ingress"
#   from_port         = 443
#   to_port           = 443
#   protocol          = "tcp"
#   security_group_id = var.jenkins_master_security_group_id
#   cidr_blocks       = ["10.0.0.0/16"]  # Adjust CIDR as per your VPC
#   description       = "Allow HTTPS traffic for SSM endpoints"
# }
# ==============================================================================

# Security Group for Bastion Host
# ==============================================================================
resource "aws_security_group" "bastion_sg" {
  vpc_id = var.vpc_id
  ingress {
    from_port        = 22
    to_port          = 22
    protocol         = "tcp"
    ipv6_cidr_blocks = ["2001:448a:2061:8ff7:78e8:3995:8f63:99a/128"] # Replace with your IP for restricted access
  }
  ingress {
    from_port        = 22
    to_port          = 22
    protocol         = "tcp"
    cidr_blocks      = ["0.0.0.0/0"] # Replace with your IP for restricted access
  }
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]  # Allow all outbound IPv4 traffic
    ipv6_cidr_blocks = ["::/0"]  # Allow all outbound IPv6 traffic
  }

  tags = {
    Name        = "bastion-sg"
    Environment = "dev"
    Terraform   = "true"
  }
}
# ==============================================================================
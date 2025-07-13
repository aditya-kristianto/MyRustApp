# EC2 Instance for Bastion Host
# ==============================================================================
resource "aws_instance" "bastion_host" {
  ami                    = "ami-015ce1b8f0618ad32"  # Replace with a suitable Amazon Linux 2 AMI ID
  instance_type          = "t4g.nano"
  subnet_id              = var.public_subnet_id
  vpc_security_group_ids = [aws_security_group.ec2_bastion_host_sg.id]
  key_name               = var.key_name
  iam_instance_profile   = var.iam_instance_profile_name

  tags = {
    Name = "bastion-host"
    Environment = "dev"
    Terraform   = "true"
  }
}
# ==============================================================================

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
  
  # egress {
  #   description      = "Allow HTTPS traffic"
  #   from_port        = 22
  #   to_port          = 22
  #   protocol         = "tcp"
  #   cidr_blocks      = var.cidr_blocks
  # }

  egress {
    description      = "Allow HTTPS traffic"
    from_port        = 443
    to_port          = 443
    protocol         = "tcp"
    cidr_blocks      = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
  }
  
  egress {
    description      = "Allow SSH traffic"
    from_port        = 22
    to_port          = 22
    protocol         = "tcp"
    cidr_blocks      = [
      "${var.private_ap_southeast_1a_cidr_block}",
      "${var.private_ap_southeast_1b_cidr_block}",
      "${var.private_ap_southeast_1c_cidr_block}"
    ]
  }
  
  egress {
    description      = "Allow HTTP traffic"
    from_port        = 8000
    to_port          = 8000
    protocol         = "tcp"
    cidr_blocks      = [
      "${var.private_ap_southeast_1a_cidr_block}",
      "${var.private_ap_southeast_1b_cidr_block}",
      "${var.private_ap_southeast_1c_cidr_block}"
    ]
  }
  
  egress {
    description      = "Allow HTTP traffic"
    from_port        = 8080
    to_port          = 8080
    protocol         = "tcp"
    cidr_blocks      = [
      "${var.private_ap_southeast_1a_cidr_block}",
      "${var.private_ap_southeast_1b_cidr_block}",
      "${var.private_ap_southeast_1c_cidr_block}"
    ]
  }
  
  egress {
    description      = "Allow HTTPS traffic"
    from_port        = 8443
    to_port          = 8443
    protocol         = "tcp"
    cidr_blocks      = [
      "${var.private_ap_southeast_1a_cidr_block}",
      "${var.private_ap_southeast_1b_cidr_block}",
      "${var.private_ap_southeast_1c_cidr_block}"
    ]
  }
  
  egress {
    description      = "Allow HTTPS traffic"
    from_port        = 9443
    to_port          = 9443
    protocol         = "tcp"
    cidr_blocks      = [
      "${var.private_ap_southeast_1a_cidr_block}",
      "${var.private_ap_southeast_1b_cidr_block}",
      "${var.private_ap_southeast_1c_cidr_block}"
    ]
  }
  
  tags = {
    Name        = "ec2_ssm_security_group"
    Environment = "dev"
    Terraform   = "true"
  }
}


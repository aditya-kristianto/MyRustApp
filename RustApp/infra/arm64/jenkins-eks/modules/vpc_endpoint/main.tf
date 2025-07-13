# VPC Endpoint for SSM
resource "aws_vpc_endpoint" "jenkins_agent_ssm" {
  vpc_id            = var.vpc_id
  service_name      = "com.amazonaws.${var.aws_region}.ssm"
  vpc_endpoint_type = "Interface"
  subnet_ids        = var.subnet_ids
  security_group_ids = [var.jenkins_agent_ec2_security_group_id]
}

# VPC Endpoint for EC2 messages (required for SSM)
resource "aws_vpc_endpoint" "jenkins_agent_ec2_messages" {
  vpc_id            = var.vpc_id
  service_name      = "com.amazonaws.${var.aws_region}.ec2messages"
  vpc_endpoint_type = "Interface"
  subnet_ids        = var.subnet_ids
  security_group_ids = [var.jenkins_agent_ec2_security_group_id]
}

# VPC Endpoint for SSM messages (required for SSM)
resource "aws_vpc_endpoint" "jenkins_agent_ssm_messages" {
  vpc_id            = var.vpc_id
  service_name      = "com.amazonaws.${var.aws_region}.ssmmessages"
  vpc_endpoint_type = "Interface"
  subnet_ids        = var.subnet_ids
  security_group_ids = [var.jenkins_agent_ec2_security_group_id]
}

# VPC Endpoint for SSM
resource "aws_vpc_endpoint" "jenkins_master_ssm" {
  vpc_id            = var.vpc_id
  service_name      = "com.amazonaws.${var.aws_region}.ssm"
  vpc_endpoint_type = "Interface"
  subnet_ids        = var.subnet_ids
  security_group_ids = [var.jenkins_master_ec2_security_group_id]
}

# VPC Endpoint for EC2 messages (required for SSM)
resource "aws_vpc_endpoint" "jenkins_master_ec2_messages" {
  vpc_id            = var.vpc_id
  service_name      = "com.amazonaws.${var.aws_region}.ec2messages"
  vpc_endpoint_type = "Interface"
  subnet_ids        = var.subnet_ids
  security_group_ids = [var.jenkins_master_ec2_security_group_id]
}

# VPC Endpoint for SSM messages (required for SSM)
resource "aws_vpc_endpoint" "jenkins_master_ssm_messages" {
  vpc_id            = var.vpc_id
  service_name      = "com.amazonaws.${var.aws_region}.ssmmessages"
  vpc_endpoint_type = "Interface"
  subnet_ids        = var.subnet_ids
  security_group_ids = [var.jenkins_master_ec2_security_group_id]
}

# VPC Endpoint for SSM
resource "aws_vpc_endpoint" "portainer_ssm" {
  vpc_id            = var.vpc_id
  service_name      = "com.amazonaws.${var.aws_region}.ssm"
  vpc_endpoint_type = "Interface"
  subnet_ids        = var.subnet_ids
  security_group_ids = [var.portainer_ec2_security_group_id]
}

# VPC Endpoint for EC2 messages (required for SSM)
resource "aws_vpc_endpoint" "portainer_ec2_messages" {
  vpc_id            = var.vpc_id
  service_name      = "com.amazonaws.${var.aws_region}.ec2messages"
  vpc_endpoint_type = "Interface"
  subnet_ids        = var.subnet_ids
  security_group_ids = [var.portainer_ec2_security_group_id]
}

# VPC Endpoint for SSM messages (required for SSM)
resource "aws_vpc_endpoint" "portainer_ssm_messages" {
  vpc_id            = var.vpc_id
  service_name      = "com.amazonaws.${var.aws_region}.ssmmessages"
  vpc_endpoint_type = "Interface"
  subnet_ids        = var.subnet_ids
  security_group_ids = [var.portainer_ec2_security_group_id]
}
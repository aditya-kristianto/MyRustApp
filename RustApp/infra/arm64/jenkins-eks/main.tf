# VPC and Networking
# ==============================================================================
module "vpc" {
  source  = "./modules/vpc"
  
  aws_region = var.aws_region
}
# ==============================================================================

# Subnet for EC2 Instance
# ==============================================================================
module "subnet" {
  source = "./modules/subnet"
  
  aws_region = var.aws_region
  vpc_id = module.vpc.vpc_id
}
# ==============================================================================

# Route53
# ==============================================================================
data "aws_route53_zone" "my_zone" {
  name         = "aditya-kristianto.com"  # Replace with your domain
  private_zone = false
}
# ==============================================================================

# Certificate Manager for Jenkins Master EC2
# ==============================================================================
module "certificate_manager" {
  source = "./modules/certificate_manager"
}
# ==============================================================================

# Route53 for EC2
# ==============================================================================
module "route53_record" {
  source  = "./modules/route53"
  
  aws_acm_certificate = module.certificate_manager.aws_acm_certificate
  # jenkins_master_record_value = module.elastic_ip.jenkins_master_public_ip
  # portainer_record_value = module.elastic_ip.portainer_public_ip
  public_ip = module.elastic_ip.nlb_eip.public_ip
  zone_id = var.zone_id
  alb_zone_id =  module.network_load_balancer.zone_id
  alb_dns_name = module.network_load_balancer.dns_name
}
# ==============================================================================

# Allocate an Elastic IP
# ==============================================================================
module "elastic_ip" {
  source = "./modules/elastic_ip"
  
  jenkins_master_instance_id = module.ec2_instance.jenkins_master_instance_id
  portainer_instance_id = module.ec2_instance.portainer_instance_id
}
# ==============================================================================

# Security Group for Jenkins Master EC2
# ==============================================================================
module "security_group" {
  source = "./modules/security_group"
  
  vpc_id = module.vpc.vpc_id
  jenkins_agent_ec2_security_group_name = var.jenkins_agent_ec2_security_group_name
  jenkins_master_ec2_security_group_name = var.jenkins_master_ec2_security_group_name
  portainer_ec2_security_group_name = var.portainer_ec2_security_group_name
  # jenkins_master_security_group_id = module.security_group.security_group_id
}
# ==============================================================================

# EC2 Instance for Jenkins Master
# ==============================================================================
module "ec2_instance" {
  source = "./modules/ec2"
  
  eks_cluster_name = module.eks.cluster_name
  jenkins_agent_iam_instance_profile = module.iam_role.jenkins_agent_iam_instance_profile
  jenkins_master_iam_instance_profile = module.iam_role.jenkins_master_iam_instance_profile
  portainer_iam_instance_profile = module.iam_role.portainer_iam_instance_profile
  public_subnet_id = module.subnet.public_subnets[0]
  subnet_id = module.subnet.public_subnets[0]
  bastion_host_security_group_id = module.security_group.bastion_host_security_group_id
  jenkins_master_security_group_id = module.security_group.jenkins_master_security_group_id
  jenkins_agent_security_group_id = module.security_group.jenkins_agent_security_group_id
  portainer_security_group_id = module.security_group.portainer_security_group_id
  use_managed_node_group = var.use_managed_node_group
  key_name = module.key_pair.key_name
  bastion_key_name = module.key_pair.bastion_key_name
}
# ==============================================================================

# VPC Endpoint for SSM in Jenkins Master EC2 Instance
# ==============================================================================
module "jenkins_master_vpc_endpoint" {
  source = "./modules/vpc_endpoint"
  
  aws_region = var.aws_region
  jenkins_agent_ec2_security_group_id = module.security_group.jenkins_agent_security_group_id
  jenkins_master_ec2_security_group_id = module.security_group.jenkins_master_security_group_id
  portainer_ec2_security_group_id = module.security_group.portainer_security_group_id
  subnet_ids = module.subnet.public_subnets
  vpc_id = module.vpc.vpc_id
}
# ==============================================================================

# IAM Role for Jenkins Master EC2 Instances
# ==============================================================================
module "iam_role" {
  source = "./modules/iam"
}
# ==============================================================================

# EKS Cluster for Jenkins Agent
# ==============================================================================
module "eks" {
  source = "./modules/eks"
  
  launch_template_id = module.ec2_instance.launch_template_id
  launch_template_name = module.ec2_instance.launch_template_name
  role_arn = module.iam_role.jenkins_agent_iam_role_role_arn
  subnet_ids = module.subnet.private_subnets
  vpc_id = module.vpc.vpc_id
  jenkins_agent_ec2_security_group_id = module.security_group.jenkins_agent_security_group_id
  
  # Pass IAM policy attachments from the IAM module as variables
  eks_worker_nodes_policy           = module.iam_role.eks_worker_nodes_policy
  eks_cni_policy                    = module.iam_role.eks_cni_policy
  ec2_container_registry_read_only  = module.iam_role.ec2_container_registry_read_only
  use_managed_node_group            = var.use_managed_node_group
}
# ==============================================================================

# EKS Cluster for Jenkins Agent
# ==============================================================================
module "network_load_balancer" {
  source = "./modules/network_load_balancer"
  
  aws_acm_certificate_arn = module.certificate_manager.certificate_arn
  private_subnets = module.subnet.private_subnets
  public_subnets = module.subnet.public_subnets
  vpc_id = module.vpc.vpc_id
  security_groups = module.security_group.security_groups
  nlb_eip = module.elastic_ip.nlb_eip
}
# ==============================================================================


# module "irsa-ebs-csi" {
#   source  = "terraform-aws-modules/iam/aws//modules/iam-assumable-role-with-oidc"
#   version = "5.39.0"

#   create_role                   = true
#   role_name                     = "AmazonEKSTFEBSCSIRole-${module.eks.cluster_name}"
#   provider_url                  = module.eks.oidc_provider
#   role_policy_arns              = [data.aws_iam_policy.ebs_csi_policy.arn]
#   oidc_fully_qualified_subjects = ["system:serviceaccount:kube-system:ebs-csi-controller-sa"]
# }

# data "aws_iam_policy" "ebs_csi_policy" {
#   arn = "arn:aws:iam::aws:policy/service-role/AmazonEBSCSIDriverPolicy"
# }

# Key Management Service for EC2 Instance
# ==============================================================================
module "key_pair" {
  source = "./modules/key_pair"
}
# ==============================================================================

# NAT Gateway for EC2 Instance with private subnet
# ==============================================================================
module "nat_gateway" {
  source = "./modules/nat_gateway"
  
  nat_eip_id = module.elastic_ip.nat_eip_id
  public_subnet = module.subnet.public_subnets[0]
}
# ==============================================================================

# Internet Gateway for EC2 Instance with private subnet
# ==============================================================================
module "internet_gateway" {
  source = "./modules/internet_gateway"
  
  vpc_id = module.vpc.vpc_id
}
# ==============================================================================

# Route Table for EC2 Instance with private subnet
# ==============================================================================
module "route_table" {
  source = "./modules/route_table"
  
  vpc_id = module.vpc.vpc_id
  internet_gateway_id = module.internet_gateway.id
  nat_gateway_id = module.nat_gateway.id
  private_subnet = module.subnet.private_subnets[0]
  public_subnet = module.subnet.public_subnets[0]
}
# ==============================================================================
# terraform {
#   required_providers {
#     aws = {
#       source  = "hashicorp/aws"
#       version = "~> 4.0"
#     }
#     tls = {
#       source  = "hashicorp/tls"
#       version = "~> 4.0"
#     }
#   }
# }

# # # Certificate Manager for Jenkins Master EC2
# # # ==============================================================================
# # module "certificate_manager" {
# #   source = "./modules/certificate_manager"
# # }
# # # ==============================================================================

# # # EKS Cluster for Jenkins Agent
# # # ==============================================================================
# # module "network_load_balancer" {
# #   source = "./modules/network_load_balancer"
  
# #   aws_acm_certificate_arn = module.certificate_manager.certificate_arn
# #   private_subnets = [
# #     "subnet-0ef5a66b4b0da2a1a",
# #     "subnet-04fb7b2562bfe65c5",
# #     "subnet-0d6618df5a3f8ace0"
# #   ]
# #   public_subnets = [
# #     "subnet-0b4641aa76c7b23ac",
# #     "subnet-09a19d00395571b8d",
# #     "subnet-0e02281a062ecd450"
# #   ]
# #   vpc_id = "vpc-0bd8aa2ff87fd5358"
# #   security_groups = ["sg-09eac05032f772a79"]
# #   nlb_eip = "eipalloc-09d7d9ac07ec4e501"
# # }
# # # ==============================================================================

# # EC2 Instance for Portainer
# # ==============================================================================
# resource "aws_instance" "portainer" {
#   ami           = "ami-05c718cb2fa58687f"
#   instance_type = "t4g.nano"
#   key_name      = aws_key_pair.my_generated_key_pair.key_name  # Attach the key pair here

#   subnet_id              = "subnet-0e17f8e9a06a8477a"
#   vpc_security_group_ids = ["sg-079fe6c8ddb427ce5"]
#   iam_instance_profile   = aws_iam_instance_profile.portainer_ec2_instance_profile.name

#   # Adding elastic IP address here
#   associate_public_ip_address = false

#   user_data = <<-EOF
#               #!/bin/bash
#               sudo yum update -y
#               sudo yum install -y amazon-ssm-agent docker
#               systemctl enable amazon-ssm-agent
#               systemctl start amazon-ssm-agent
#               systemctl start docker
#               systemctl enable docker  # Enable Docker to start on boot

#               # Pull Jenkins Docker image and run it
#               docker volume create portainer_data
#               docker run -d --name portainer \
#                 -p 80:8000 \
#                 -p 443:9443 \
#                 --restart=always \
#                 -v /var/run/docker.sock:/var/run/docker.sock \
#                 -v portainer_data:/data \
#                 portainer/portainer-ce:2.21.4
#               EOF
              
#   # Adding a 50GB EBS volume
#   root_block_device {
#     volume_size = 10                # Set the volume size to 10GB
#     volume_type = "gp3"             # General Purpose SSD (gp3 or gp2)
#     delete_on_termination = true    # Delete the volume when the instance is terminated
#   }
  
#   tags = {
#     Environment = "dev"
#     Name        = "portainer"
#     Terraform   = "true"
#   }
# }
# # ==============================================================================

# resource "aws_iam_role_policy_attachment" "portainer_ec2_role_for_ssm_policy" {
#   role       = aws_iam_role.portainer_iam_role.name
#   policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonEC2RoleforSSM"
# }

# resource "aws_iam_role_policy_attachment" "portainer_ssm_managed_policy" {
#   role       = aws_iam_role.portainer_iam_role.name
#   policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
# }

# resource "aws_iam_instance_profile" "portainer_ec2_instance_profile" {
#   name = "portainer-ec2-instance-profile"
#   role = aws_iam_role.portainer_iam_role.name
# }

# # IAM Role for Portainer EC2 Instances
# # ==============================================================================
# resource "aws_iam_role" "portainer_iam_role" {
#   name = "portainer-iam-role"

#   assume_role_policy = jsonencode({
#     Version = "2012-10-17"
#     Statement = [
#       {
#         Action    = "sts:AssumeRole"
#         Effect    = "Allow"
#         Principal = {
#           Service = [
# 			      "ec2.amazonaws.com"
# 		      ]
#         }
#       },
#     ]
#   })
  
#   tags = {
#     Environment = "dev"
#     Terraform   = "true"
#   }
# }
# # ==============================================================================

# # VPC Endpoint for SSM
# resource "aws_vpc_endpoint" "portainer_ssm" {
#   vpc_id            = "vpc-01ffd72db391a7910"
#   service_name      = "com.amazonaws.ap-southeast-1.ssm"
#   vpc_endpoint_type = "Interface"
#   subnet_ids        = ["subnet-0e17f8e9a06a8477a"]
#   security_group_ids = ["sg-079fe6c8ddb427ce5"]
# }

# # VPC Endpoint for EC2 messages (required for SSM)
# resource "aws_vpc_endpoint" "portainer_ec2_messages" {
#   vpc_id            = "vpc-01ffd72db391a7910"
#   service_name      = "com.amazonaws.ap-southeast-1.ec2messages"
#   vpc_endpoint_type = "Interface"
#   subnet_ids        = ["subnet-0e17f8e9a06a8477a"]
#   security_group_ids = ["sg-079fe6c8ddb427ce5"]
# }

# # VPC Endpoint for SSM messages (required for SSM)
# resource "aws_vpc_endpoint" "portainer_ssm_messages" {
#   vpc_id            = "vpc-01ffd72db391a7910"
#   service_name      = "com.amazonaws.ap-southeast-1.ssmmessages"
#   vpc_endpoint_type = "Interface"
#   subnet_ids        = ["subnet-0e17f8e9a06a8477a"]
#   security_group_ids = ["sg-079fe6c8ddb427ce5"]
# }

# resource "aws_security_group_rule" "allow_ssm_https_inbound" {
#   type            = "ingress"
#   from_port       = 443
#   to_port         = 443
#   protocol        = "tcp"
#   security_group_id = "sg-079fe6c8ddb427ce5"
#   cidr_blocks     = ["0.0.0.0/0"]
#   description     = "Allow inbound HTTPS for SSM"
# }

# resource "aws_security_group_rule" "allow_ssm_https_outbound" {
#   type            = "egress"
#   from_port       = 443
#   to_port         = 443
#   protocol        = "tcp"
#   security_group_id = "sg-079fe6c8ddb427ce5"
#   cidr_blocks     = ["0.0.0.0/0"]
#   description     = "Allow outbound HTTPS for SSM"
# }

# # Alternatively, to generate a new key pair and output the private key
# resource "tls_private_key" "my_generated_key" {
#   algorithm = "RSA"
#   rsa_bits  = 2048
# }

# resource "aws_key_pair" "my_generated_key_pair" {
#   key_name   = "my-portainer-key-pair"
#   public_key = tls_private_key.my_generated_key.public_key_openssh
# }

# resource "local_file" "private_key_file" {
#   content  = tls_private_key.my_generated_key.private_key_pem
#   filename = "${path.module}/my_generated_key.pem"
# }

# resource "local_file" "public_key_file" {
#   content  = tls_private_key.my_generated_key.public_key_openssh
#   filename = "${path.module}/my_generated_key.pub"
# }

# output "path_module" {
#   description = "The path of module"
#   value       = "${path.module}"
# }



module "vpc" {
  source = "./modules/vpc"
}

module "application_load_balancer" {
  source = "./modules/application_load_balancer"
  
  certificate_arn           = module.certificate_manager.certificate_arn
  public_subnets            = module.vpc.public_subnets
  vpc_id                    = module.vpc.id
  portainer_private_ip      = module.portainer.private_ip
  jenkins_master_private_ip = module.jenkins_master.private_ip
  jenkins_master_target_id  = module.jenkins_master.target_id
  portainer_target_id       = module.portainer.target_id
  pgadmin_private_ip        = module.postgres.private_ip
  rust_app_private_ip       = module.rust_app.private_ip
  
  depends_on = [ module.certificate_manager ]
}

module "certificate_manager" {
  source = "./modules/certificate_manager"
}

module "iam" {
  source = "./modules/iam"
    
  vpc_id = module.vpc.id
}

module "key_pair" {
  source = "./modules/key_pair"
}

module "bastion_host" {
  source = "./modules/bastion_host"

  cidr_blocks                         = module.vpc.private_subnets
  private_ap_southeast_1a_cidr_block  = module.vpc.private_ap_southeast_1a_cidr_block
  private_ap_southeast_1b_cidr_block  = module.vpc.private_ap_southeast_1b_cidr_block
  private_ap_southeast_1c_cidr_block  = module.vpc.private_ap_southeast_1c_cidr_block
  public_subnet_id                    = module.vpc.private_ap_southeast_1a
  vpc_id                              = module.vpc.id
  key_name                            = module.key_pair.name
  iam_instance_profile_name           = module.iam.ssm_profile_name
}

# module "jenkins_agent" {
#   source = "./modules/jenkins_agent"
  
#   private_subnet_ids  = module.vpc.private_subnets
#   private_subnet_1_id = module.vpc.private_subnet_1_id
#   private_subnet_2_id = module.vpc.private_subnet_2_id
#   private_subnet_3_id = module.vpc.private_subnet_3_id
#   public_subnet_1_id  = module.vpc.public_subnet_1_id
#   public_subnet_2_id  = module.vpc.public_subnet_2_id
#   public_subnet_3_id  = module.vpc.public_subnet_3_id
#   vpc_id              = module.vpc.id
# }

module "jenkins_master" {
  source = "./modules/jenkins_master"
  
  cidr_blocks                     = ["${module.bastion_host.private_ip}/32"]
  iam_instance_profile_name       = module.iam.ssm_profile_name
  key_name                        = module.key_pair.name
  private_subnet_id               = module.vpc.private_ap_southeast_1a
  vpc_id                          = module.vpc.id
}

module "portainer" {
  source = "./modules/portainer"
  
  cidr_blocks                     = ["${module.bastion_host.private_ip}/32"]
  private_subnet_id               = module.vpc.private_ap_southeast_1a
  vpc_id                          = module.vpc.id
  key_name                        = module.key_pair.name
  iam_instance_profile_name       = module.iam.ssm_profile_name
}

# module "test2" {
#   source = "./modules/test-2"
# }

module "test3" {
  source = "./modules/test-3"
  
  private_subnet_ids      = module.vpc.private_subnets
  private_ap_southeast_1a = module.vpc.private_ap_southeast_1a
  private_ap_southeast_1b = module.vpc.private_ap_southeast_1b
  private_ap_southeast_1c = module.vpc.private_ap_southeast_1c
  public_ap_southeast_1a  = module.vpc.public_ap_southeast_1a
  public_ap_southeast_1b  = module.vpc.public_ap_southeast_1b
  public_ap_southeast_1c  = module.vpc.public_ap_southeast_1c
  vpc_id                  = module.vpc.id
}

module "postgres" {
  source = "./modules/postgres"
  
  cidr_blocks               = ["${module.bastion_host.private_ip}/32"]
  iam_instance_profile_name = module.iam.ssm_profile_name
  key_name                  = module.key_pair.name
  private_subnet_id         = module.vpc.private_ap_southeast_1a
  vpc_id                    = module.vpc.id
}

module "rust_app" {
  source = "./modules/rust_app"
  
  cidr_blocks               = ["${module.bastion_host.private_ip}/32"]
  iam_instance_profile_name = module.iam.ssm_profile_name
  key_name                  = module.key_pair.name
  postgres_instance_id      = module.postgres.instance_id
  postgres_private_ip       = module.postgres.private_ip
  private_subnet_id         = module.vpc.private_ap_southeast_1a
  vpc_id                    = module.vpc.id
  
  depends_on = [ time_sleep.wait ]
}

resource "time_sleep" "wait" {
  depends_on = [ module.postgres ]

  create_duration = "5m"
}

# module "security_group" {
#   source = "./modules/security_group"
  
#   vpc_id = module.vpc.id
#   bastion_host_cidr_blocks = [
#       aws_subnet.private_subnet_1.cidr_block,
#       aws_subnet.private_subnet_2.cidr_block,
#       aws_subnet.private_subnet_3.cidr_block
#     ]
#   portainer_cidr_blocks = [aws_subnet.private_subnet_1.cidr_block]
# }

# module "ec2" {
#   source = "./modules/ec2"
  
#   private_subnet_id               = aws_subnet.private_subnet_1.id
#   public_subnet_id                = aws_subnet.public_subnet_1.id
#   bastion_host_security_group_ids = [module.security_group.ec2_bastion_host_sg_id]
#   portainer_security_group_ids    = [module.security_group.ec2_portainer_sg_id]
#   key_name                        = aws_key_pair.my_key_pair.key_name
#   iam_instance_profile_name       = aws_iam_instance_profile.ssm_profile.name
# }

# module "subnet" {
#   source  = "./modules/subnet"
  
#   vpc_id = aws_vpc.my_vpc.id
# }

# module "elastic_ip" {
#   source  = "./modules/elastic_ip"
# }

# module "internet_gateway" {
#   source  = "./modules/internet_gateway"
  
#   vpc_id = aws_vpc.my_vpc.id
# }

# resource "aws_vpc_endpoint" "ssm_endpoint" {
#   vpc_id            = module.vpc.id
#   service_name      = "com.amazonaws.ap-southeast-1.ssm"
#   vpc_endpoint_type = "Interface"  # Change to "Interface"
#   subnet_ids        = [
#     aws_subnet.private_subnet_1.id, 
#     aws_subnet.private_subnet_2.id, 
#     aws_subnet.private_subnet_3.id
#   ]
#   security_group_ids = [aws_security_group.vpc_endpoint_sg.id]
  
#   tags = {
#     Name = "SSM Endpoint"
#   }
# }

# resource "aws_vpc_endpoint" "ec2_messages_endpoint" {
#   vpc_id            = module.vpc.id
#   service_name      = "com.amazonaws.ap-southeast-1.ec2messages"
#   vpc_endpoint_type = "Interface"  # Change to "Interface"
#   subnet_ids        = [
#     aws_subnet.private_subnet_1.id, 
#     aws_subnet.private_subnet_2.id, 
#     aws_subnet.private_subnet_3.id
#   ]
#   security_group_ids = [aws_security_group.vpc_endpoint_sg.id]
  
#   tags = {
#     Name = "EC2 Messages Endpoint"
#   }
# }

# resource "aws_vpc_endpoint" "ssm_messages_endpoint" {
#   vpc_id            = module.vpc.id
#   service_name      = "com.amazonaws.ap-southeast-1.ssmmessages"
#   vpc_endpoint_type = "Interface"  # Change to "Interface"
#   subnet_ids        = [
#     aws_subnet.private_subnet_1.id, 
#     aws_subnet.private_subnet_2.id, 
#     aws_subnet.private_subnet_3.id
#   ]
#   security_group_ids = [aws_security_group.vpc_endpoint_sg.id]
  
#   tags = {
#     Name = "SSM Messages Endpoint"
#   }
# }


# resource "aws_subnet" "public_subnet_1" {
#   vpc_id            = module.vpc.id
#   cidr_block        = "10.0.101.0/24"
#   availability_zone = "ap-southeast-1a"
#   map_public_ip_on_launch = true

#   tags = {
#     Name        = "my-vpc-public-subnet-1"
#     Environment = "dev"
#     Terraform   = "true"
#   }
# }

# resource "aws_subnet" "private_subnet_1" {
#   vpc_id            = module.vpc.id
#   cidr_block        = "10.0.1.0/24"
#   availability_zone = "ap-southeast-1a"

#   tags = {
#     Name        = "my-vpc-private-subnet-1"
#     Environment = "dev"
#     Terraform   = "true"
#   }
# }

# resource "aws_subnet" "public_subnet_2" {
#   vpc_id            = module.vpc.id
#   cidr_block        = "10.0.102.0/24"
#   availability_zone = "ap-southeast-1b"
#   map_public_ip_on_launch = true

#   tags = {
#     Name        = "my-vpc-public-subnet-2"
#     Environment = "dev"
#     Terraform   = "true"
#   }
# }

# resource "aws_subnet" "private_subnet_2" {
#   vpc_id            = module.vpc.id
#   cidr_block        = "10.0.2.0/24"
#   availability_zone = "ap-southeast-1b"

#   tags = {
#     Name        = "my-vpc-private-subnet-2"
#     Environment = "dev"
#     Terraform   = "true"
#   }
# }

# resource "aws_subnet" "public_subnet_3" {
#   vpc_id            = module.vpc.id
#   cidr_block        = "10.0.103.0/24"
#   availability_zone = "ap-southeast-1c"
#   map_public_ip_on_launch = true

#   tags = {
#     Name        = "my-vpc-public-subnet-3"
#     Environment = "dev"
#     Terraform   = "true"
#   }
# }

# resource "aws_subnet" "private_subnet_3" {
#   vpc_id            = module.vpc.id
#   cidr_block        = "10.0.3.0/24"
#   availability_zone = "ap-southeast-1c"

#   tags = {
#     Name        = "my-vpc-private-subnet-3"
#     Environment = "dev"
#     Terraform   = "true"
#   }
# }

# resource "aws_eip" "nat_eip" {
#   domain = "vpc"
# }

# # Internet Gateway for NAT
# resource "aws_internet_gateway" "my_igw" {
#   vpc_id =  module.vpc.id
# }

# resource "aws_nat_gateway" "nat_gw" {
#   allocation_id = aws_eip.nat_eip.id
#   subnet_id     = aws_subnet.private_subnet_1.id
# }

# Public Route Table (for NAT Gateway)
# resource "aws_route_table" "public_route_table" {
#   vpc_id = module.vpc.id
# }

# resource "aws_route" "public_route" {
#   route_table_id         = aws_route_table.public_route_table.id
#   destination_cidr_block = "0.0.0.0/0"
#   gateway_id             = aws_internet_gateway.my_igw.id
# }

# resource "aws_route_table_association" "public_subnet_1_association" {
#   subnet_id      = aws_subnet.public_subnet_1.id
#   route_table_id = aws_route_table.public_route_table.id
  
#   lifecycle {
#     create_before_destroy = true
#   }
# }

# resource "aws_route_table_association" "public_subnet_2_association" {
#   subnet_id      = aws_subnet.public_subnet_2.id
#   route_table_id = aws_route_table.public_route_table.id

#   lifecycle {
#     create_before_destroy = true
#   }
# }

# resource "aws_route_table_association" "public_subnet_3_association" {
#   subnet_id      = aws_subnet.public_subnet_3.id
#   route_table_id = aws_route_table.public_route_table.id

#   lifecycle {
#     create_before_destroy = true
#   }
# }

# Private Route Table (routes through NAT)
# resource "aws_route_table" "private_route_table" {
#   vpc_id = module.vpc.id
# }

# resource "aws_route" "private_route" {
#   route_table_id         = aws_route_table.private_route_table.id
#   destination_cidr_block = "0.0.0.0/0"
#   nat_gateway_id         = aws_nat_gateway.nat_gw.id
# }

# resource "aws_route_table_association" "private_subnet_1_association" {
#   subnet_id      = aws_subnet.private_subnet_1.id
#   route_table_id = aws_route_table.private_route_table.id
  
#   lifecycle {
#     create_before_destroy = true
#   }
# }

# resource "aws_route_table_association" "private_subnet_2_association" {
#   subnet_id      = aws_subnet.private_subnet_2.id
#   route_table_id = aws_route_table.private_route_table.id

#   lifecycle {
#     create_before_destroy = true
#   }
# }

# resource "aws_route_table_association" "private_subnet_3_association" {
#   subnet_id      = aws_subnet.private_subnet_3.id
#   route_table_id = aws_route_table.private_route_table.id

#   lifecycle {
#     create_before_destroy = true
#   }
# }


# Step 3: Launch EC2 instance with SSM access


# resource "aws_security_group" "jenkins_agent_sg" {
#   name        = "jenkins-agent-sg"
#   description = "Security group for Jenkins Agent EC2 instance"
#   vpc_id      = module.vpc.id

#   ingress {
#     description = "HTTP"
#     from_port   = 80
#     to_port     = 80
#     protocol    = "tcp"
#     cidr_blocks = ["0.0.0.0/0"]
#   }
  
#   ingress {
#     from_port   = 443
#     to_port     = 443
#     protocol    = "tcp"
#     cidr_blocks = ["0.0.0.0/0"]
#   }

#   ingress {
#     from_port   = 10250
#     to_port     = 10250
#     protocol    = "tcp"
#     cidr_blocks = ["0.0.0.0/0"]
#   }
  
#   ingress {
#     from_port        = 0
#     to_port          = 0
#     protocol         = "-1"
#     cidr_blocks      = ["0.0.0.0/0"]
#   }

#   egress {
#     description = "Allow all outbound traffic"
#     from_port   = 0
#     to_port     = 0
#     protocol    = "-1"
#     cidr_blocks = ["0.0.0.0/0"]
#   }

#   tags = {
#     Name        = "jenkins-agent-sg"
#     Environment = "dev"
#     Terraform   = "true"
#   }
# }

# resource "aws_eks_cluster" "jenkins_agent_eks_cluster" {
#   name     = "jenkins-agent-eks-cluster"
#   role_arn = aws_iam_role.jenkins_agent_iam_role.arn
  
#   version = "1.31"

#   # VPC configuration: attach EKS to these subnets
#   vpc_config {
#     subnet_ids              = [
#       aws_subnet.private_subnet_1.id, 
#       aws_subnet.private_subnet_2.id, 
#       aws_subnet.private_subnet_3.id
#     ]
#     security_group_ids      = [aws_security_group.jenkins_agent_sg.id]
#     endpoint_public_access  = true            # Enable public access to the EKS API
#     endpoint_private_access = true            # Enable private access to the EKS API (optional but recommended)
#   }

#   # Enable Kubernetes API server logging
#   # enabled_cluster_log_types = ["api", "audit", "authenticator"]
  
#   tags = {
#     Environment = "dev"
#     Terraform   = "true"
#   }
  
#   depends_on = [
#     aws_iam_role_policy_attachment.eks_worker_nodes_policy,
#     aws_iam_role_policy_attachment.eks_cni_policy,
#     aws_iam_role_policy_attachment.ec2_container_registry_read_only,
#   ]
# }

# EC2 Launch Template for Jenkins Agent
# ==============================================================================
# resource "aws_launch_template" "jenkins_agent_launch_template" {
#   name                   = "jenkins-agent-eks-node-launch-template"
#   image_id               = "ami-08a49464b2384edd9"
#   instance_type          = "t4g.small"
#   ebs_optimized          = true
#   update_default_version = true
#   # vpc_security_group_ids = [var.jenkins_agent_security_group_id]
  
#   # Add any additional configuration here, such as key name, user data, etc.
#   # key_name = var.key_name
  
#   # iam_instance_profile {
#   #   name = var.jenkins_agent_iam_instance_profile
#   # }
  
#   monitoring {
#     enabled = true
#   }
  
#   network_interfaces {
#     associate_public_ip_address = false
#     security_groups             = [aws_security_group.jenkins_agent_sg.id]
#     # subnet_id                   = var.subnet_id
#   }

#   # Use user_data to install the SSM Agent
#   user_data = base64encode(<<EOF
#     #!/bin/bash
#     # Install SSM agent
#     yum update -y
#     yum install -y amazon-ssm-agent
#     systemctl enable amazon-ssm-agent
#     systemctl start amazon-ssm-agent
#     /etc/eks/bootstrap.sh ${aws_eks_cluster.jenkins_agent_eks_cluster.name} --kubelet-extra-args '--node-labels=node.kubernetes.io/lifecycle=on-demand'
#   EOF
#   )
  
#   block_device_mappings {
#     device_name = "/dev/xvda"
    
#     ebs {
#       volume_size = 20
#       volume_type = "gp3"
#     }
#   }
  
#   tag_specifications {
#     resource_type = "instance"
#     tags = {
#       Environment = "dev"
#       Name        = "jenkins-agent"
#       Terraform   = "true"
#     }
#   }
# }
# ==============================================================================

# resource "aws_iam_role" "jenkins_agent_iam_role" {
#   name = "jenkins-agent-iam-role"

#   assume_role_policy = jsonencode({
#     Version = "2012-10-17"
#     Statement = [
#       {
#         Action    = "sts:AssumeRole"
#         Effect    = "Allow"
#         Principal = {
#           Service = [
# 			      "ec2.amazonaws.com",
# 		        "eks.amazonaws.com"
# 		      ]
#         }
#       },
#     ]
#   })
  
#   tags = {
#     Environment = "dev"
#     Terraform   = "true"
#   }
# }

# resource "aws_iam_role_policy_attachment" "eks_worker_nodes_policy" {
#   role       = aws_iam_role.jenkins_agent_iam_role.name
#   policy_arn = "arn:aws:iam::aws:policy/AmazonEKSWorkerNodePolicy"
# }

# resource "aws_iam_role_policy_attachment" "eks_cni_policy" {
#   role       = aws_iam_role.jenkins_agent_iam_role.name
#   policy_arn = "arn:aws:iam::aws:policy/AmazonEKS_CNI_Policy"
# }

# resource "aws_iam_role_policy_attachment" "ec2_container_registry_read_only" {
#   role       = aws_iam_role.jenkins_agent_iam_role.name
#   policy_arn = "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly"
# }

# EKS Cluster for Jenkins Agent
# ==============================================================================
# module "eks" {
#   source          = "terraform-aws-modules/eks/aws"
#   version         = "20.24.3"
  
#   cluster_name    = "jenkins-agent-eks-cluster"
#   cluster_version = "1.31" # Choose the desired Kubernetes version
  
#   cluster_endpoint_public_access           = true
#   enable_cluster_creator_admin_permissions = true
  
#   subnet_ids      = [aws_subnet.private_subnet_1.id,aws_subnet.private_subnet_2.id,aws_subnet.private_subnet_3.id]
#   vpc_id          = module.vpc.id
  
#   cluster_addons = {
#     aws-ebs-csi-driver      = {
#       service_account_role_arn = module.irsa-ebs-csi.iam_role_arn
#     }
#     vpc-cni                 = {
#       addon_name               = "vpc-cni"
#       addon_version            = "v1.18.5-eksbuild.1"   # Specify the version of VPC CNI
#       resolve_conflicts        = "OVERWRITE"
#       service_account_role_arn = null
#     }
#     coredns                 = {
#       addon_name               = "coredns"
#       addon_version            = "v1.11.3-eksbuild.1"    # Specify the version of CoreDNS
#       resolve_conflicts        = "OVERWRITE"
#       service_account_role_arn = null
#     }
#     kube-proxy              = {
#       addon_name               = "kube-proxy"
#       addon_version            = "v1.31.1-eksbuild.2"   # Specify the version of kube-proxy
#       resolve_conflicts        = "OVERWRITE"
#       service_account_role_arn = null
#     }
#     eks-pod-identity-agent  = {
#       addon_name               = "eks-pod-identity-agent"
#       addon_version            = "v1.3.2-eksbuild.2"   # Specify the version of kube-proxy
#       resolve_conflicts        = "OVERWRITE"
#       service_account_role_arn = null
#     }
#   }
  
#   eks_managed_node_group_defaults = {
#     ami_type = "AL2_ARM_64"
#   }

#   eks_managed_node_groups = {
#     one = {
#       name = "jenkins-agent"

#       instance_types = ["t4g.small"]

#       min_size     = 1
#       max_size     = 2
#       desired_size = 1
      
#       asg_desired_capacity = 1   # Set to 0 to turn off instances
#       asg_min_size         = 1
#       asg_max_size         = 2
      
#       # Attach the launch template with user data
#       launch_template = {
#         id      = aws_launch_template.jenkins_agent_launch_template.id
#         version = "$Latest"
#       }
#     }
#   }
  
#   # Ensure that IAM Role permissions are created before and deleted after EKS Node Group handling.
#   # Otherwise, EKS will not be able to properly delete EC2 Instances and Elastic Network Interfaces.
#   depends_on = [
#     aws_iam_role_policy_attachment.eks_worker_nodes_policy,
#     aws_iam_role_policy_attachment.eks_cni_policy,
#     aws_iam_role_policy_attachment.ec2_container_registry_read_only,
#   ]

#   tags = {
#     Environment = "dev"
#     Terraform   = "true"
#   }
# }

# module "irsa-ebs-csi" {
#   source  = "terraform-aws-modules/iam/aws//modules/iam-assumable-role-with-oidc"
#   version = "5.39.0"

#   create_role                   = true
#   role_name                     = "AmazonEKSTFEBSCSIRole-${module.eks.cluster_name}"
#   provider_url                  = module.eks.oidc_provider
#   role_policy_arns              = ["arn:aws:iam::aws:policy/service-role/AmazonEBSCSIDriverPolicy"]
#   oidc_fully_qualified_subjects = ["system:serviceaccount:kube-system:ebs-csi-controller-sa"]
# }
# EKS Cluster for Jenkins Agent
# ==============================================================================
module "eks" {
  source          = "terraform-aws-modules/eks/aws"
  version         = "20.24.3"
  
  cluster_name    = "jenkins-agent-eks-cluster"
  cluster_version = "1.31" # Choose the desired Kubernetes version
  
  cluster_endpoint_public_access           = true
  enable_cluster_creator_admin_permissions = true
  
  subnet_ids      = var.private_subnet_ids
  vpc_id          = var.vpc_id
  
  cluster_addons = {
    aws-ebs-csi-driver      = {
      service_account_role_arn = module.irsa-ebs-csi.iam_role_arn
    }
    vpc-cni                 = {
      addon_name               = "vpc-cni"
      addon_version            = "v1.18.5-eksbuild.1"   # Specify the version of VPC CNI
      resolve_conflicts        = "OVERWRITE"
      service_account_role_arn = null
    }
    coredns                 = {
      addon_name               = "coredns"
      addon_version            = "v1.11.3-eksbuild.1"    # Specify the version of CoreDNS
      resolve_conflicts        = "OVERWRITE"
      service_account_role_arn = null
    }
    kube-proxy              = {
      addon_name               = "kube-proxy"
      addon_version            = "v1.31.1-eksbuild.2"   # Specify the version of kube-proxy
      resolve_conflicts        = "OVERWRITE"
      service_account_role_arn = null
    }
    eks-pod-identity-agent  = {
      addon_name               = "eks-pod-identity-agent"
      addon_version            = "v1.3.2-eksbuild.2"   # Specify the version of kube-proxy
      resolve_conflicts        = "OVERWRITE"
      service_account_role_arn = null
    }
  }
  
  eks_managed_node_group_defaults = {
    ami_type = "AL2_ARM_64"
  }

  eks_managed_node_groups = {
    one = {
      name = "jenkins-agent"

      instance_types = ["t4g.small"]

      min_size     = 1
      max_size     = 2
      desired_size = 1
      
      asg_desired_capacity = 1   # Set to 0 to turn off instances
      asg_min_size         = 1
      asg_max_size         = 2
      
      # Attach the launch template with user data
      launch_template = {
        id      = aws_launch_template.jenkins_agent_launch_template.id
        version = "$Latest"
      }
    }
  }
  
  # Ensure that IAM Role permissions are created before and deleted after EKS Node Group handling.
  # Otherwise, EKS will not be able to properly delete EC2 Instances and Elastic Network Interfaces.
  depends_on = [
    aws_iam_role_policy_attachment.eks_worker_nodes_policy,
    aws_iam_role_policy_attachment.eks_cni_policy,
    aws_iam_role_policy_attachment.ec2_container_registry_read_only,
  ]

  tags = {
    Environment = "dev"
    Terraform   = "true"
  }
}

# Security Group for Jenkins Agent EC2
# ==============================================================================
resource "aws_security_group" "jenkins_agent_sg" {
  name        = "jenkins-agent-sg"
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

resource "aws_iam_role_policy_attachment" "eks_worker_nodes_policy" {
  role       = aws_iam_role.jenkins_agent_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonEKSWorkerNodePolicy"
}

resource "aws_iam_role_policy_attachment" "eks_cni_policy" {
  role       = aws_iam_role.jenkins_agent_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonEKS_CNI_Policy"
}

resource "aws_iam_role_policy_attachment" "ec2_container_registry_read_only" {
  role       = aws_iam_role.jenkins_agent_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly"
}

resource "aws_iam_role_policy_attachment" "eks_cluster_policy" {
  role       = aws_iam_role.jenkins_agent_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonEKSClusterPolicy"
}

resource "aws_iam_role_policy_attachment" "eks_service_policy" {
  role       = aws_iam_role.jenkins_agent_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonEKSVPCResourceController"
}

# IAM Role for Jenkins Agent EC2 Instances
# ==============================================================================
resource "aws_iam_role" "jenkins_agent_iam_role" {
  name = "jenkins-agent-iam-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action    = "sts:AssumeRole"
        Effect    = "Allow"
        Principal = {
          Service = [
		    "ec2.amazonaws.com",
	        "eks.amazonaws.com"
	      ]
        }
      },
    ]
  })
  
  tags = {
    Environment = "dev"
    Terraform   = "true"
  }
}
# ==============================================================================

# EC2 Launch Template for Jenkins Agent
# ==============================================================================
resource "aws_launch_template" "jenkins_agent_launch_template" {
  name                   = "jenkins-agent-eks-node-launch-template"
  image_id               = "ami-08a49464b2384edd9"
  instance_type          = "t4g.small"
  ebs_optimized          = true
  update_default_version = true
  # vpc_security_group_ids = [var.jenkins_agent_security_group_id]
  
  # Add any additional configuration here, such as key name, user data, etc.
  # key_name = var.key_name
  
  # iam_instance_profile {
  #   name = var.jenkins_agent_iam_instance_profile
  # }
  
  monitoring {
    enabled = true
  }
  
  network_interfaces {
    associate_public_ip_address = false
    security_groups             = [aws_security_group.jenkins_agent_sg.id]
    # subnet_id                   = var.subnet_id
  }

  # Use user_data to install the SSM Agent
  user_data = base64encode(<<EOF
    #!/bin/bash
    # Install SSM agent
    yum update -y
    yum install -y amazon-ssm-agent
    systemctl enable amazon-ssm-agent
    systemctl start amazon-ssm-agent
    /etc/eks/bootstrap.sh ${module.eks.cluster_name} --kubelet-extra-args '--node-labels=node.kubernetes.io/lifecycle=on-demand'
  EOF
  )
  
  block_device_mappings {
    device_name = "/dev/xvda"
    
    ebs {
      volume_size = 20
      volume_type = "gp3"
    }
  }
  
  tag_specifications {
    resource_type = "instance"
    tags = {
      Environment = "dev"
      Name        = "jenkins-agent"
      Terraform   = "true"
    }
  }
}
# ==============================================================================

module "irsa-ebs-csi" {
  source  = "terraform-aws-modules/iam/aws//modules/iam-assumable-role-with-oidc"
  version = "5.39.0"

  create_role                   = true
  role_name                     = "AmazonEKSTFEBSCSIRole-${module.eks.cluster_name}"
  provider_url                  = module.eks.oidc_provider
  role_policy_arns              = ["arn:aws:iam::aws:policy/service-role/AmazonEBSCSIDriverPolicy"]
  oidc_fully_qualified_subjects = ["system:serviceaccount:kube-system:ebs-csi-controller-sa"]
}
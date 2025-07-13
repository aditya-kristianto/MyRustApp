# EKS Cluster for Jenkins Agent
# ==============================================================================
module "eks" {
  count = var.use_managed_node_group ? 1 : 0
  
  source          = "terraform-aws-modules/eks/aws"
  version         = "20.24.3"
  
  cluster_name    = "jenkins-agent-eks-cluster"
  cluster_version = "1.31" # Choose the desired Kubernetes version
  
  cluster_endpoint_public_access           = true
  enable_cluster_creator_admin_permissions = true
  
  subnet_ids      = var.subnet_ids
  vpc_id          = var.vpc_id
  
  cluster_addons = {
    aws-ebs-csi-driver      = {
      service_account_role_arn = module.irsa-ebs-csi[0].iam_role_arn
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
        id      = var.launch_template_id
        version = "$Latest"
      }
    }
  }
  
  # Ensure that IAM Role permissions are created before and deleted after EKS Node Group handling.
  # Otherwise, EKS will not be able to properly delete EC2 Instances and Elastic Network Interfaces.
  depends_on = [
    var.eks_worker_nodes_policy,
    var.eks_cni_policy,
    var.ec2_container_registry_read_only,
  ]

  tags = {
    Environment = "dev"
    Terraform   = "true"
  }
}

resource "aws_eks_cluster" "jenkins_agent_eks_cluster" {
  count = !var.use_managed_node_group ? 1 : 0  # Create only if managed node group is selected

  name     = "jenkins-agent-eks-cluster"
  role_arn = var.role_arn
  
  version = "1.31"

  # VPC configuration: attach EKS to these subnets
  vpc_config {
    subnet_ids              = var.subnet_ids
    security_group_ids      = [var.jenkins_agent_ec2_security_group_id]
    endpoint_public_access  = false            # Enable public access to the EKS API
    endpoint_private_access = true            # Enable private access to the EKS API (optional but recommended)
  }

  # Enable Kubernetes API server logging
  # enabled_cluster_log_types = ["api", "audit", "authenticator"]
  
  tags = {
    Environment = "dev"
    Terraform   = "true"
  }
  
  depends_on = [
    var.eks_worker_nodes_policy,
    var.eks_cni_policy,
    var.ec2_container_registry_read_only,
  ]
}
# ==============================================================================

module "irsa-ebs-csi" {
  count = var.use_managed_node_group ? 1 : 0  # Create only if managed node group is selected
  
  source  = "terraform-aws-modules/iam/aws//modules/iam-assumable-role-with-oidc"
  version = "5.39.0"

  create_role                   = true
  role_name                     = "AmazonEKSTFEBSCSIRole-${module.eks[0].cluster_name}"
  provider_url                  = module.eks[0].oidc_provider
  role_policy_arns              = [data.aws_iam_policy.ebs_csi_policy.arn]
  oidc_fully_qualified_subjects = ["system:serviceaccount:kube-system:ebs-csi-controller-sa"]
}

# https://aws.amazon.com/blogs/containers/amazon-ebs-csi-driver-is-now-generally-available-in-amazon-eks-add-ons/ 
data "aws_iam_policy" "ebs_csi_policy" {
  arn = "arn:aws:iam::aws:policy/service-role/AmazonEBSCSIDriverPolicy"
}

resource "aws_eks_addon" "vpc_cni" {
  count = !var.use_managed_node_group ? 1 : 0  # Create only if managed node group is selected
  
  cluster_name                = aws_eks_cluster.jenkins_agent_eks_cluster[0].name
  addon_name                  = "vpc-cni"
  addon_version               = "v1.18.5-eksbuild.1"
  resolve_conflicts_on_update = "OVERWRITE"
}

resource "aws_eks_addon" "coredns" {
  count = !var.use_managed_node_group ? 1 : 0  # Create only if managed node group is selected
  
  cluster_name                = aws_eks_cluster.jenkins_agent_eks_cluster[0].name
  addon_name                  = "coredns"
  addon_version               = "v1.11.3-eksbuild.1"
  resolve_conflicts_on_update = "OVERWRITE"
  
  timeouts {
    create = "10m"
  }
}

resource "aws_eks_addon" "kube_proxy" {
  count = !var.use_managed_node_group ? 1 : 0  # Create only if managed node group is selected
  
  cluster_name                = aws_eks_cluster.jenkins_agent_eks_cluster[0].name
  addon_name                  = "kube-proxy"
  addon_version               = "v1.31.1-eksbuild.2"
  resolve_conflicts_on_update = "OVERWRITE"
}

resource "aws_eks_addon" "eks_pod_identity_agent" {
  count = !var.use_managed_node_group ? 1 : 0  # Create only if managed node group is selected
  
  cluster_name                = aws_eks_cluster.jenkins_agent_eks_cluster[0].name
  addon_name                  = "eks-pod-identity-agent"
  addon_version               = "v1.3.2-eksbuild.2"
  resolve_conflicts_on_update = "OVERWRITE"
}

# EKS Node Group for Jenkins Agent
# ==============================================================================
# resource "aws_eks_node_group" "jenkins_agent_node_group" {
#   count = !var.use_managed_node_group ? 1 : 0  # Create only if managed node group is selected

#   cluster_name    = aws_eks_cluster.jenkins_agent_eks_cluster[0].name
#   node_group_name = "jenkins-agent-node-group"
#   node_role_arn   = var.role_arn
#   subnet_ids      = var.subnet_ids

#   scaling_config {
#     desired_size = 1
#     max_size     = 2
#     min_size     = 1
#   }
  
#   update_config {
#     max_unavailable = 1
#   }

#   # Specify the instance type and other settings
#   # instance_types = ["t4g.small"]

#   # Specify user_data to install SSM Agent
#   launch_template {
#     name    = var.launch_template_name
#     version = "$Latest"  # You can use "$Latest" or a specific version number
#   }

#   tags = {
#     Environment = "dev"
#     Terraform   = "true"
#   }
  
#   # Ensure that IAM Role permissions are created before and deleted after EKS Node Group handling.
#   # Otherwise, EKS will not be able to properly delete EC2 Instances and Elastic Network Interfaces.
#   depends_on = [
#     var.eks_worker_nodes_policy,
#     var.eks_cni_policy,
#     var.ec2_container_registry_read_only,
#   ]
# }
# ==============================================================================
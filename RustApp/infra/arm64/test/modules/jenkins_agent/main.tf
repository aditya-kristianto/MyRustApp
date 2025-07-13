# EKS Cluster for Jenkins Agent
# ==============================================================================
resource "aws_eks_cluster" "jenkins_agent_cluster" {
  name     = "jenkins-agent-cluster"
  role_arn = aws_iam_role.jenkins_agent_cluster_iam_role.arn

  vpc_config {
    subnet_ids = [
      var.private_subnet_1_id,
      var.private_subnet_2_id,
      var.private_subnet_3_id,
      var.public_subnet_1_id,
      var.public_subnet_2_id,
      var.public_subnet_3_id,
    ]
  }

  depends_on = [aws_iam_role_policy_attachment.AmazonEKSClusterPolicy]
}

resource "aws_iam_role" "jenkins_agent_cluster_iam_role" {
  name = "jenkins-agent-cluster-iam-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action    = "sts:AssumeRole"
        Effect    = "Allow"
        Principal = {
          Service = [
			      "eks.amazonaws.com"
		      ]
        }
      }]
  })
}

resource "aws_iam_role_policy_attachment" "AmazonEKSClusterPolicy" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonEKSClusterPolicy"
  role       = aws_iam_role.jenkins_agent_cluster_iam_role.name
}

resource "aws_eks_addon" "vpc_cni" {
  cluster_name                = aws_eks_cluster.jenkins_agent_cluster.name
  addon_name                  = "vpc-cni"
  addon_version               = "v1.19.0-eksbuild.1"
  resolve_conflicts_on_update = "OVERWRITE"
}

resource "aws_eks_addon" "coredns" {
  cluster_name                = aws_eks_cluster.jenkins_agent_cluster.name
  addon_name                  = "coredns"
  addon_version               = "v1.11.3-eksbuild.1"
  resolve_conflicts_on_update = "OVERWRITE"
}

resource "aws_eks_addon" "kube_proxy" {
  cluster_name                = aws_eks_cluster.jenkins_agent_cluster.name
  addon_name                  = "kube-proxy"
  addon_version               = "v1.31.2-eksbuild.2"
  resolve_conflicts_on_update = "OVERWRITE"
}

resource "aws_eks_addon" "eks_pod_identity_agent" {
  cluster_name                = aws_eks_cluster.jenkins_agent_cluster.name
  addon_name                  = "eks-pod-identity-agent"
  addon_version               = "v1.3.2-eksbuild.2"
  resolve_conflicts_on_update = "OVERWRITE"
}
# ==============================================================================

# EKS Node Group for Jenkins Agent
# ==============================================================================
resource "aws_eks_node_group" "jenkins_agent_nodes" {
  cluster_name    = aws_eks_cluster.jenkins_agent_cluster.name
  node_group_name = "jenkins-agent-nodes"
  node_role_arn   = aws_iam_role.jenkins_agent_nodes_iam_role.arn

  subnet_ids = [
    var.private_subnet_1_id,
    var.private_subnet_2_id,
    var.private_subnet_3_id
  ]

  capacity_type   = "ON_DEMAND"
  ami_type        = "AL2_ARM_64"
  instance_types  = ["t4g.nano"]

  scaling_config {
    desired_size = 1
    max_size     = 2
    min_size     = 0
  }

  update_config {
    max_unavailable = 1
  }

  labels = {
    role = "general"
  }
  
  tags = {
    Name = "jenkins-agent-nodes"
  }

  # taint {
  #   key    = "team"
  #   value  = "devops"
  #   effect = "NO_SCHEDULE"
  # }

  # launch_template {
  #   name    = aws_launch_template.eks-with-disks.name
  #   version = aws_launch_template.eks-with-disks.latest_version
  # }

  depends_on = [
    aws_iam_role_policy_attachment.AmazonEKSWorkerNodePolicy,
    aws_iam_role_policy_attachment.AmazonEKS_CNI_Policy,
    aws_iam_role_policy_attachment.AmazonEC2ContainerRegistryReadOnly,
  ]
}

resource "aws_iam_role" "jenkins_agent_nodes_iam_role" {
  name = "jenkins-agent-node-group-iam-role"

  assume_role_policy = jsonencode({
    Statement = [{
      Action = "sts:AssumeRole"
      Effect = "Allow"
      Principal = {
        Service = "ec2.amazonaws.com"
      }
    }]
    Version = "2012-10-17"
  })
}

resource "aws_iam_role_policy_attachment" "AmazonEKSWorkerNodePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonEKSWorkerNodePolicy"
  role       = aws_iam_role.jenkins_agent_nodes_iam_role.name
}

resource "aws_iam_role_policy_attachment" "AmazonEKS_CNI_Policy" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonEKS_CNI_Policy"
  role       = aws_iam_role.jenkins_agent_nodes_iam_role.name
}

resource "aws_iam_role_policy_attachment" "AmazonEC2ContainerRegistryReadOnly" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly"
  role       = aws_iam_role.jenkins_agent_nodes_iam_role.name
}
# ==============================================================================
# EKS Cluster for Jenkins Agent
# ==============================================================================
resource "aws_eks_cluster" "jenkins_agent_cluster" {
  name            = "jenkins_agent_cluster"
  role_arn        = aws_iam_role.jenkins_agent_cluster.arn

  vpc_config {
    subnet_ids = [
      var.private_ap_southeast_1a,
      var.private_ap_southeast_1b,
      var.private_ap_southeast_1c,
      var.public_ap_southeast_1a,
      var.public_ap_southeast_1b,
      var.public_ap_southeast_1c
    ]
  }

  depends_on = [aws_iam_role_policy_attachment.jenkins_agent_cluster-AmazonEKSClusterPolicy]
}

resource "aws_iam_role" "jenkins_agent_cluster" {
  name = "eks-cluster-jenkins_agent_cluster"

  assume_role_policy = jsonencode({
    "Version": "2012-10-17",
    "Statement": [
      {
        "Effect": "Allow",
        "Principal": {
          "Service": "eks.amazonaws.com"
        },
        "Action": "sts:AssumeRole"
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "jenkins_agent_cluster-AmazonEKSClusterPolicy" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonEKSClusterPolicy"
  role       = aws_iam_role.jenkins_agent_cluster.name
}

resource "aws_eks_addon" "vpc_cni" {
  cluster_name                = aws_eks_cluster.jenkins_agent_cluster.name
  addon_name                  = "vpc-cni"
  addon_version               = "v1.19.0-eksbuild.1"
  resolve_conflicts_on_update = "OVERWRITE"
}

# resource "aws_eks_addon" "coredns" {
#  cluster_name                = aws_eks_cluster.jenkins_agent_cluster.name
#  addon_name                  = "coredns"
#  addon_version               = "v1.11.3-eksbuild.1"
#  resolve_conflicts_on_update = "OVERWRITE"
  
#  depends_on = [
#    aws_eks_node_group.jenkins_agent_nodes,
#    aws_eks_cluster.jenkins_agent_cluster
#  ]
#}

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
  node_role_arn   = aws_iam_role.nodes.arn

  subnet_ids = [
    var.private_ap_southeast_1a,
    var.private_ap_southeast_1b,
    var.private_ap_southeast_1c
  ]

  capacity_type       = "ON_DEMAND"
  ami_type            = "AL2_ARM_64"
  instance_types      = ["t4g.nano"]

  scaling_config {
    desired_size = 1
    max_size     = 5
    min_size     = 0
  }

  update_config {
    max_unavailable = 1
  }

  labels = {
    role = "general"
  }

  launch_template {
    name    = aws_launch_template.jenkins_agent_launch_template.name
    version = aws_launch_template.jenkins_agent_launch_template.latest_version
  }

  depends_on = [
    aws_iam_role_policy_attachment.nodes-AmazonEKSWorkerNodePolicy,
    aws_iam_role_policy_attachment.nodes-AmazonEKS_CNI_Policy,
    aws_iam_role_policy_attachment.nodes-AmazonEC2ContainerRegistryReadOnly,
  ]
}

resource "aws_launch_template" "jenkins_agent_launch_template" {
  name = "jenkins-agent-launch-template"

  ebs_optimized           = true
  update_default_version  = true
  key_name                = aws_key_pair.jenkins_agent_key_pair.key_name
  
  # network_interfaces {
  #   associate_public_ip_address = false
  #   security_groups             = [aws_security_group.jenkins_agent_sg.id]
  #   # subnet_id                   = var.subnet_id
  # }

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

resource "aws_iam_role" "nodes" {
  name = "eks-node-group-nodes"

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

resource "aws_iam_role_policy_attachment" "nodes-AmazonEKSWorkerNodePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonEKSWorkerNodePolicy"
  role       = aws_iam_role.nodes.name
}

resource "aws_iam_role_policy_attachment" "nodes-AmazonEKS_CNI_Policy" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonEKS_CNI_Policy"
  role       = aws_iam_role.nodes.name
}

resource "aws_iam_role_policy_attachment" "nodes-AmazonEC2ContainerRegistryReadOnly" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly"
  role       = aws_iam_role.nodes.name
}

# Alternatively, to generate a new key pair and output the private key
resource "tls_private_key" "jenkins_agent_key" {
  algorithm = "RSA"
  rsa_bits  = 2048
}

resource "aws_key_pair" "jenkins_agent_key_pair" {
  key_name   = "my-jenkins-agent-key-pair"
  public_key = tls_private_key.jenkins_agent_key.public_key_openssh
}

resource "local_file" "jenkins_agent_private_key_file" {
  content  = tls_private_key.jenkins_agent_key.private_key_pem
  filename = "${path.module}/jenkins_agent_key.pem"
}

resource "local_file" "jenkins_agent_public_key_file" {
  content  = tls_private_key.jenkins_agent_key.public_key_openssh
  filename = "${path.module}/jenkins_agent_key.pub"
}

# data "tls_certificate" "eks" {
#   url = aws_eks_cluster.jenkins_agent_cluster.identity[0].oidc[0].issuer
# }

# resource "aws_iam_openid_connect_provider" "eks" {
#   client_id_list  = ["sts.amazonaws.com"]
#   thumbprint_list = [data.tls_certificate.eks.certificates[0].sha1_fingerprint]
#   url             = aws_eks_cluster.jenkins_agent_cluster.identity[0].oidc[0].issuer
# }

# data "aws_iam_policy_document" "test_oidc_assume_role_policy" {
#   statement {
#     actions = ["sts:AssumeRoleWithWebIdentity"]
#     effect  = "Allow"

#     condition {
#       test     = "StringEquals"
#       variable = "${replace(aws_iam_openid_connect_provider.eks.url, "https://", "")}:sub"
#       values   = ["system:serviceaccount:default:aws-test"]
#     }

#     principals {
#       identifiers = [aws_iam_openid_connect_provider.eks.arn]
#       type        = "Federated"
#     }
#   }
# }

# resource "aws_iam_role" "test_oidc" {
#   assume_role_policy = data.aws_iam_policy_document.test_oidc_assume_role_policy.json
#   name               = "test-oidc"
# }

# resource "aws_iam_policy" "test-policy" {
#   name = "test-policy"

#   policy = jsonencode({
#     Statement = [{
#       Action = [
#         "s3:ListAllMyBuckets",
#         "s3:GetBucketLocation"
#       ]
#       Effect   = "Allow"
#       Resource = "arn:aws:s3:::*"
#     }]
#     Version = "2012-10-17"
#   })
# }

# resource "aws_iam_role_policy_attachment" "test_attach" {
#   role       = aws_iam_role.test_oidc.name
#   policy_arn = aws_iam_policy.test-policy.arn
# }

# output "test_policy_arn" {
#   value = aws_iam_role.test_oidc.arn
# }

resource "aws_security_group" "jenkins_agent_sg" {
  name        = "jenkins-agent-sg"
  description = "Security group for Jenkins Agent EC2 instance"
  vpc_id      = var.vpc_id

  ingress {
    description = "SSH"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name        = "jenkins-agent-sg"
    Environment = "dev"
    Terraform   = "true"
  }
}
# ==============================================================================
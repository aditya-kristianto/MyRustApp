# IAM Role for Jenkins Master EC2 Instances
# ==============================================================================
resource "aws_iam_role" "jenkins_master_iam_role" {
  name = "jenkins-master-iam-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action    = "sts:AssumeRole"
        Effect    = "Allow"
        Principal = {
          Service = "ec2.amazonaws.com"
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

# IAM Role for Portainer EC2 Instances
# ==============================================================================
resource "aws_iam_role" "portainer_iam_role" {
  name = "portainer-iam-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action    = "sts:AssumeRole"
        Effect    = "Allow"
        Principal = {
          Service = [
			      "ec2.amazonaws.com"
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

# Attach required policies for SSM Session Manager
resource "aws_iam_role_policy_attachment" "jenkins_master_ssm_managed_policy" {
  role       = aws_iam_role.jenkins_master_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

resource "aws_iam_role_policy_attachment" "jenkins_agent_ssm_managed_policy" {
  role       = aws_iam_role.jenkins_agent_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

resource "aws_iam_role_policy_attachment" "portainer_ssm_managed_policy" {
  role       = aws_iam_role.portainer_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

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

resource "aws_iam_instance_profile" "jenkins_agent_ec2_instance_profile" {
  name = "jenkins-agent-ec2-instance-profile"
  role = aws_iam_role.jenkins_agent_iam_role.name
}

resource "aws_iam_instance_profile" "jenkins_master_ec2_instance_profile" {
  name = "jenkins-master-ec2-instance-profile"
  role = aws_iam_role.jenkins_master_iam_role.name
}

resource "aws_iam_instance_profile" "portainer_ec2_instance_profile" {
  name = "portainer-ec2-instance-profile"
  role = aws_iam_role.portainer_iam_role.name
}
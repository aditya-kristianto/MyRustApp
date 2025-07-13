# Step 1: Create IAM Role for SSM
resource "aws_iam_role" "ssm_role" {
  name = "ec2_ssm_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [{
      Action    = "sts:AssumeRole",
      Effect    = "Allow",
      Principal = {
        Service = "ec2.amazonaws.com"
      }
    }]
  })
}

resource "aws_iam_policy" "s3_access_policy" {
  name        = "ec2-s3-access-policy"
  description = "Allow EC2 instance to read from S3"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect   = "Allow"
        Action   = [
          "s3:GetObject",
          "s3:PutObject"
        ]
        Resource = "arn:aws:s3:::rust-app-bucket/databases/rust_app/backups/*"
      }
    ]
  })
}

resource "aws_iam_policy" "ecr_access_policy" {
  name        = "ECRAccessPolicy"
  description = "Policy to allow EC2 to authenticate and pull images from ECR"
  
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action   = [
          "ecr:GetDownloadUrlForLayer",
				  "ecr:BatchGetImage",
				  "ecr:DescribeImages",
				  "ecr:ListImages",
				  "ecr:BatchCheckLayerAvailability",
				  "ecr:GetRegistryPolicy",
          "ecr:DescribeRegistry",
          "ecr:DescribePullThroughCacheRules",
          "ecr:GetAuthorizationToken",
          "ecr:GetRegistryScanningConfiguration",
          "ecr:BatchImportUpstreamImage",
        ]
        Effect   = "Allow"
        Resource = "arn:aws:ecr:ap-southeast-1:473154593366:repository/*"
      }
    ]
  })
}

# Attach the SSM Managed Policy
resource "aws_iam_role_policy_attachment" "ssm_managed_policy" {
  role       = aws_iam_role.ssm_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

resource "aws_iam_role_policy_attachment" "attach_ecr_access" {
  role       = aws_iam_role.ssm_role.name
  policy_arn = aws_iam_policy.ecr_access_policy.arn
}

resource "aws_iam_role_policy_attachment" "attach_s3_policy" {
  role       = aws_iam_role.ssm_role.name
  policy_arn = aws_iam_policy.s3_access_policy.arn
}

resource "aws_security_group" "vpc_endpoint_sg" {
  name        = "vpc_endpoint_security_group"
  description = "Allow SSM access"
  vpc_id      = var.vpc_id  # Replace with your VPC ID

  ingress {
    description      = "Allow HTTPS traffic"
    from_port        = 443
    to_port          = 443
    protocol         = "tcp"
    cidr_blocks      = ["0.0.0.0/0"]  # Replace with your VPC CIDR range
    ipv6_cidr_blocks = ["::/0"]
  }
  
  tags = {
    Name        = "vpc_endpoint_security_group"
    Environment = "dev"
    Terraform   = "true"
  }
}

# Step 2: Create a Security Group allowing inbound SSM access


# Step 4: IAM Instance Profile
resource "aws_iam_instance_profile" "ssm_profile" {
  name = "ssm_instance_profile"
  role = aws_iam_role.ssm_role.name
}
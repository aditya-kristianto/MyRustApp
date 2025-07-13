# EC2 Instance for Bastion Host
# ==============================================================================
resource "aws_instance" "bastion_host" {
  ami             = "ami-05c718cb2fa58687f" # Amazon Linux 2 AMI (Change to latest for your region)
  instance_type   = "t4g.nano"              # Use a small instance type like t3.nano or t3.micro
  key_name        = var.bastion_key_name
  subnet_id       = var.public_subnet_id
  security_groups = [var.bastion_host_security_group_id]
  
  associate_public_ip_address = true

  tags = {
    Name = "bastion-host"
  }
}
# ==============================================================================

# EC2 Instance for Portainer
# ==============================================================================
resource "aws_instance" "portainer" {
  ami           = "ami-05c718cb2fa58687f"
  instance_type = "t4g.nano"
  key_name      = var.key_name

  subnet_id              = var.subnet_id
  vpc_security_group_ids = [var.portainer_security_group_id]
  iam_instance_profile   = var.portainer_iam_instance_profile

  # Adding elastic IP address here
  associate_public_ip_address = false

  user_data = <<-EOF
              #!/bin/bash
              sudo yum update -y
              sudo yum install -y amazon-ssm-agent docker
              systemctl enable amazon-ssm-agent
              systemctl start amazon-ssm-agent
              systemctl start docker
              systemctl enable docker  # Enable Docker to start on boot

              # Pull Jenkins Docker image and run it
              docker volume create portainer_data
              docker run -d --name portainer \
                -p 80:8000 \
                -p 443:9443 \
                --restart=always \
                -v /var/run/docker.sock:/var/run/docker.sock \
                -v portainer_data:/data \
                portainer/portainer-ce:2.21.4
              EOF
              
  # Adding a 50GB EBS volume
  root_block_device {
    volume_size = 10                # Set the volume size to 10GB
    volume_type = "gp3"             # General Purpose SSD (gp3 or gp2)
    delete_on_termination = true    # Delete the volume when the instance is terminated
  }
  
  tags = {
    Environment = "dev"
    Name        = "portainer"
    Terraform   = "true"
  }
}
# ==============================================================================

# EC2 Instance for Jenkins Master
# ==============================================================================
resource "aws_instance" "jenkins_master" {
  ami           = "ami-05c718cb2fa58687f"
  instance_type = "t4g.small"
  key_name      = var.key_name

  subnet_id              = var.subnet_id
  vpc_security_group_ids = [var.jenkins_master_security_group_id]
  iam_instance_profile   = var.jenkins_master_iam_instance_profile

  # Adding elastic IP address here
  associate_public_ip_address = false

  user_data = <<-EOF
              #!/bin/bash
              sudo yum update -y
              sudo yum install -y amazon-ssm-agent docker git java
              systemctl enable amazon-ssm-agent
              systemctl start amazon-ssm-agent
              systemctl start docker
              systemctl enable docker  # Enable Docker to start on boot

              # Pull Jenkins Docker image and run it
              docker run --name jenkins \
                -p 80:8080 \
                -p 443:8443 \
                -p 50000:50000 \
                --restart unless-stopped \
                --cpus="1" \
                --memory="1.5g" \
                --memory-swap="1.5g" \
                -d jenkins/jenkins:2.482-slim-jdk21
                
              mount -o remount,size=5G /tmp/
              
              ssh-keygen -t rsa -b 4096 -C "kristianto.aditya@gmail.com"
              
              # Install Jenkins plugins
              cat <<EOT > /usr/share/jenkins/ref/plugins.txt
              kubernetes:1.35.3
              credentials-binding:1.27
              workflow-aggregator:2.6
              git:4.13.0
              pipeline-stage-view:2.23
              EOT
              # sudo wget -O /usr/share/jenkins/ref/plugins.txt https://raw.githubusercontent.com/your-repo/plugins.txt
              sudo /usr/bin/install-plugins.sh < /usr/share/jenkins/ref/plugins.txt
              EOF
              
  # Adding a 50GB EBS volume
  root_block_device {
    volume_size = 50                # Set the volume size to 50GB
    volume_type = "gp3"             # General Purpose SSD (gp3 or gp2)
    delete_on_termination = true    # Delete the volume when the instance is terminated
  }

  # provisioner "local-exec" {
  #   command = "echo 'Jenkins Master EC2 instance (${aws_instance.jenkins_master.public_ip}) is up and running.'"
  # }
  
  tags = {
    Environment = "dev"
    Name        = "jenkins-master"
    Terraform   = "true"
  }
}
# ==============================================================================

# EC2 Launch Template for Jenkins Agent
# ==============================================================================
resource "aws_launch_template" "jenkins_agent_launch_template" {
  count = !var.use_managed_node_group ? 1 : 0  # Create only if managed node group is selected

  name                   = "jenkins-agent-eks-node-launch-template"
  image_id               = "ami-08a49464b2384edd9"
  instance_type          = "t4g.small"
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
    security_groups             = [var.jenkins_agent_security_group_id]
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
    /etc/eks/bootstrap.sh ${var.eks_cluster_name} --kubelet-extra-args '--node-labels=node.kubernetes.io/lifecycle=on-demand'
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

# Create an SSM Document to get the Jenkins password
resource "aws_ssm_document" "jenkins_exec" {
  name          = "JenkinsExecDocument"
  document_type = "Command"
  content = jsonencode({
    schemaVersion = "2.2"
    description   = "Execute command to get Jenkins initial password"
    mainSteps = [
      {
        action = "aws:runShellScript"
        name   = "runShellCommand"
        inputs = {
          DocumentName = "AWS-RunShellScript"
          Parameters = {
            commands = [
              "docker exec jenkins cat /var/jenkins_home/secrets/initialAdminPassword"
            ]
          }
        }
      }
    ]
  })
  
  depends_on = [
    aws_instance.jenkins_master
  ]
}
# Allocate an Elastic IP
# ==============================================================================
resource "aws_eip" "nlb_eip" {
  domain = "vpc"  # Make sure it's in the same VPC
  
  tags = {
    Name        = "my-eip"
    Terraform   = "true"
    Environment = "dev"
  }
}
# resource "aws_eip" "jenkins_master_elastic_ip" {
#   domain = "vpc"  # Make sure it's in the same VPC
  
#   tags = {
#     Terraform   = "true"
#     Environment = "dev"
#   }
# }

# resource "aws_eip" "portainer_elastic_ip" {
#   domain = "vpc"  # Make sure it's in the same VPC
  
#   tags = {
#     Terraform   = "true"
#     Environment = "dev"
#   }
# }
# ==============================================================================

# Associate the Elastic IP with the Jenkins Master EC2 instance
# ==============================================================================
# resource "aws_eip_association" "jenkins_master_eip_assoc" {
#   allocation_id = aws_eip.jenkins_master_elastic_ip.id
#   instance_id = var.jenkins_master_instance_id
# }

# resource "aws_eip_association" "portainer_eip_assoc" {
#   allocation_id = aws_eip.portainer_elastic_ip.id
#   instance_id = var.portainer_instance_id
# }
# ==============================================================================

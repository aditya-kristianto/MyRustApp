# output "jenkins_agent_cluster_endpoint" {
#   description = "EKS Cluster Endpoint"
#   value       = aws_eks_cluster.jenkins_agent_eks_cluster.endpoint
# }

# output "kubeconfig_file" {
#   description = "Path to the generated kubeconfig file."
#   value       = local_file.kubeconfig.filename
# }

# output "node_group_arn" {
#   description = "ARN of the EKS Node Group."
#   value       = module.eks.managed_node_groups["one"].arn
# }

output "jenkins_master_ec2_public_ip" {
  description = "Public IP of the Jenkins master EC2 instance."
  value       = module.ec2_instance.jenkins_master_public_ip
}

output "portainer_ec2_public_ip" {
  description = "Public IP of the Jenkins master EC2 instance."
  value       = module.ec2_instance.portainer_public_ip
}

# output "jenkins_master_security_group_id" {
#   description = "Security Group ID for Jenkins Master."
#   value       = module.jenkins_master_security_group.security_group_id
# }

output "vpc_id" {
  description = "VPC ID."
  value       = module.vpc.vpc_id
}

output "public_subnets" {
  description = "List of public subnets."
  value       = module.subnet.public_subnets
}

output "private_subnets" {
  description = "List of public subnets."
  value       = module.subnet.private_subnets
}

output "certificate_arn" {
  value = module.certificate_manager.certificate_arn
}

# output "node_iam_role" {
#   value = module.eks.eks_managed_node_groups["one"].iam_role_arn
# }

# output "kubernetes_version" {
#   description = "The Kubernetes version of the EKS cluster."
#   value       = data.aws_eks_cluster.jenkins_cluster.version
# }

# output "helm_version" {
#   value = data.external.helm_version.result["stdout"]
# }
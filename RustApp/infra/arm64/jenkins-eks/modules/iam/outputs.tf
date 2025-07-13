output "jenkins_agent_iam_role_role_arn" {
  description = "The IAM Role ARN of the Jenkins Agent EC2 Instance"
  value       = aws_iam_role.jenkins_agent_iam_role.arn
}

output "role_name" {
  description = "The IAM Role Name of the Jenkins Master EC2 Instance"
  value       = aws_iam_role.jenkins_master_iam_role.name
}

output "jenkins_agent_iam_instance_profile" {
  description = "The IAM Instance Profile Name of the Jenkins Master EC2 Instance"
  value       = aws_iam_instance_profile.jenkins_agent_ec2_instance_profile.name
}

output "jenkins_master_iam_instance_profile" {
  description = "The IAM Instance Profile Name of the Jenkins Master EC2 Instance"
  value       = aws_iam_instance_profile.jenkins_master_ec2_instance_profile.name
}

output "portainer_iam_instance_profile" {
  description = "The IAM Instance Profile Name of the Portainer EC2 Instance"
  value       = aws_iam_instance_profile.portainer_ec2_instance_profile.name
}

output "eks_worker_nodes_policy" {
  value = aws_iam_role_policy_attachment.eks_worker_nodes_policy
}

output "eks_cni_policy" {
  value = aws_iam_role_policy_attachment.eks_cni_policy
}

output "ec2_container_registry_read_only" {
  value = aws_iam_role_policy_attachment.ec2_container_registry_read_only
}

output "bastion_host_security_group_id" {
  description = "The Security Group ID of the Bastion Host EC2 Security Group"
  value       = aws_security_group.bastion_sg.id
}

output "jenkins_agent_security_group_id" {
  description = "The Security Group ID of the Jenkins Master EC2 Security Group"
  value       = aws_security_group.jenkins_agent_sg.id
}

output "jenkins_master_security_group_id" {
  description = "The Security Group ID of the Jenkins Master EC2 Security Group"
  value       = aws_security_group.jenkins_master_sg.id
}

output "portainer_security_group_id" {
  description = "The Security Group ID of the Portainer EC2 Security Group"
  value       = aws_security_group.portainer_sg.id
}

output "security_groups" {
  description = "The Security Group ID of the EC2 Security Group"
  value       = [
    aws_security_group.jenkins_agent_sg.id,
    aws_security_group.jenkins_master_sg.id,
    aws_security_group.portainer_sg.id,
    aws_security_group.bastion_sg.id
  ]
}
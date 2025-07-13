output "jenkins_master_instance_id" {
  description = "The Public IP of the Jenkins Master EC2 Instance ID"
  value       = aws_instance.jenkins_master.id
}

output "portainer_instance_id" {
  description = "The Public IP of the Portainer EC2 Instance ID"
  value       = aws_instance.portainer.id
}

output "launch_template_id" {
  description = "The Launch Template Name of the EC2 Launch Template"
  value       = var.use_managed_node_group ? "" : aws_launch_template.jenkins_agent_launch_template[0].id
}

output "launch_template_name" {
  description = "The Launch Template Name of the EC2 Launch Template"
  value       = var.use_managed_node_group ? "" : aws_launch_template.jenkins_agent_launch_template[0].name
}

output "jenkins_master_public_ip" {
  description = "The Public IP of the Jenkins Master EC2 Instance"
  value       = aws_instance.jenkins_master.public_ip
}

output "portainer_public_ip" {
  description = "The Public IP of the Jenkins Master EC2 Instance"
  value       = aws_instance.portainer.public_ip
}

output "bastion_host_public_ip" {
  value = aws_instance.bastion_host.public_ip
}

# Output the password (optional, can be set as sensitive)
# output "jenkins_initial_password" {
#   value     = aws_ssm_command.jenkins_command.output
#   sensitive = true
# }
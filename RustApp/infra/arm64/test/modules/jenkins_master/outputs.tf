output "private_ip" {
    value = aws_instance.jenkins_master.private_ip
    description = "The Private IP Address of the Jenkins Master."
}

output "target_id" {
    value = aws_instance.jenkins_master.id
    description = "The ID of the Jenkins Master."
}

output "certificate_content" {
    value = local_file.certificate_file.content
}

output "jenkins_master_private_key_content" {
    value = local_file.jenkins_master_private_key_file.content
}

output "jenkins_master_public_key_file" {
    value = local_file.certificate_file.content
}
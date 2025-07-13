output "private_ip" {
    value = aws_instance.portainer.private_ip
    description = "The Private IP Address of the Portainer."
}

output "target_id" {
    value = aws_instance.portainer.id
    description = "The ID of the Portainer."
}
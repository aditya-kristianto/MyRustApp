output "bastion_host_instance_private_ip" {
  value = aws_instance.bastion_host.private_ip
}

output "portainer_instance_private_ip" {
  value = aws_instance.portainer.private_ip
}
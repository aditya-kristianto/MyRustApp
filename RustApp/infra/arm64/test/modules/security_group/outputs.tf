output "ec2_bastion_host_sg_id" {
    value = aws_security_group.ec2_bastion_host_sg.id
    description = "The Security Group of the Bastion Host."
}

output "ec2_portainer_sg_id" {
    value = aws_security_group.ec2_portainer_sg.id
    description = "The Security Group of the Portainer."
}
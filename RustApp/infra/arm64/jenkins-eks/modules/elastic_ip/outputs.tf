# output "jenkins_master_public_ip" {
#   description = "The Public IP of the Elastic IP"
#   value       = aws_eip.jenkins_master_elastic_ip.public_ip
# }

# output "portainer_public_ip" {
#   description = "The Public IP of the Elastic IP"
#   value       = aws_eip.portainer_elastic_ip.public_ip
# }

output "nlb_eip" {
  description = "The Public IP of the Elastic IP"
  value       = aws_eip.nlb_eip
}

output "nat_eip_id" {
  description = "The ID of the Elastic IP"
  value       = aws_eip.nlb_eip.id
}
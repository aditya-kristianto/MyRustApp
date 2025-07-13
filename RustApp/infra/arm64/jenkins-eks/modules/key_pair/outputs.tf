output "key_name" {
  description = "The Key Management Service Name"
  value       = aws_key_pair.my_generated_key_pair.key_name
}

output "bastion_key_name" {
  description = "The Key Management Service Name"
  value       = aws_key_pair.bastion_key.key_name
}

output "bastion_private_key" {
  value     = tls_private_key.bastion_key.private_key_pem
  sensitive = true
}
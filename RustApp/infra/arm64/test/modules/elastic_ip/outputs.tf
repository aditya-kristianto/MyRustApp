output "id" {
  description = "The ID of the Elastic IP"
  value       = aws_eip.nat_eip.id
}
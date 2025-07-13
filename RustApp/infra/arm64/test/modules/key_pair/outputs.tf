output "name" {
  description = "The Key Pair Name"
  value       = aws_key_pair.my_key_pair.key_name
}
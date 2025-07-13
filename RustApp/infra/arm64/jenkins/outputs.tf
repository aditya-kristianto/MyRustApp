output "elastic_ip" {
  description = "The elastic IP Address"
  value = aws_eip.elastic_ip.public_ip
}
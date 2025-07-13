output "instance_id" {
    value = aws_instance.postgres.id
    description = "The Instance ID of the Postgres."
}

output "private_ip" {
    value = aws_instance.postgres.private_ip
    description = "The Private IP Address of the PGAdmin."
}
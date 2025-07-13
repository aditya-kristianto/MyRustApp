output "private_ip" {
    value = aws_instance.rust_app.private_ip
    description = "The Private IP Address of the Rust App."
}
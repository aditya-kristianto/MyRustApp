# Create an SSH Key Pair (Optional: if you want Terraform to manage the key)
resource "tls_private_key" "bastion_key" {
  algorithm = "RSA"
  rsa_bits  = 2048
}

resource "aws_key_pair" "bastion_key" {
  key_name   = "bastion-key"
  public_key = tls_private_key.bastion_key.public_key_openssh
}

# Alternatively, to generate a new key pair and output the private key
resource "tls_private_key" "my_generated_key" {
  algorithm = "RSA"
  rsa_bits  = 2048
}

resource "aws_key_pair" "my_generated_key_pair" {
  key_name   = "my-key-pair"
  public_key = tls_private_key.my_generated_key.public_key_openssh
}

resource "local_file" "private_key_file" {
  content  = tls_private_key.my_generated_key.private_key_pem
  filename = "${path.module}/my_generated_key.pem"
}

resource "local_file" "public_key_file" {
  content  = tls_private_key.my_generated_key.public_key_openssh
  filename = "${path.module}/my_generated_key.pub"
}

resource "local_file" "bastion_private_key_file" {
  content  = tls_private_key.bastion_key.private_key_pem
  filename = "${path.module}/bastion-key.pem"
}

resource "local_file" "bastion_public_key_file" {
  content  = tls_private_key.bastion_key.public_key_openssh
  filename = "${path.module}/bastion-key.pub"
}
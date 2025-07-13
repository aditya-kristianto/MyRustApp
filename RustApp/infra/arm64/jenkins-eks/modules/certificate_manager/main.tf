# Certificate Manager for Jenkins Master EC2
# ==============================================================================
resource "aws_acm_certificate" "my_certificate" {
  domain_name       = "aditya-kristianto.com"   # Your domain
  validation_method = "DNS"

  subject_alternative_names = [
    "jenkins.aditya-kristianto.com",
    "portainer.aditya-kristianto.com" # Add any additional subdomains
  ]
  
  lifecycle {
    create_before_destroy = true
  }

  validation_option {
    domain_name       = "jenkins.aditya-kristianto.com"
    validation_domain = "aditya-kristianto.com"
  }
  
  validation_option {
    domain_name       = "portainer.aditya-kristianto.com"
    validation_domain = "aditya-kristianto.com"
  }
  
  tags = {
    Name = "My SSL Certificate"
  }
}
# ==============================================================================
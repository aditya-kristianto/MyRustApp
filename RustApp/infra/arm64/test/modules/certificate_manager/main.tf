# Certificate Manager for Jenkins Master EC2
# ==============================================================================
resource "aws_acm_certificate" "my_certificate" {
  domain_name       = "aditya-kristianto.com"   # Your domain
  validation_method = "DNS"

  subject_alternative_names = [
    "jenkins.aditya-kristianto.com",
    "portainer.aditya-kristianto.com",
    "rust-app.aditya-kristianto.com",
    "rust-auth-app.aditya-kristianto.com",
    "rust-oauth-app.aditya-kristianto.com",
    "rust-stock-app.aditya-kristianto.com",
    "rust-uuid-app.aditya-kristianto.com" # Add any additional subdomains
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
  
  validation_option {
    domain_name       = "rust-app.aditya-kristianto.com"
    validation_domain = "aditya-kristianto.com"
  }
  
   validation_option {
    domain_name       = "rust-auth-app.aditya-kristianto.com"
    validation_domain = "aditya-kristianto.com"
  }
  
   validation_option {
    domain_name       = "rust-oauth-app.aditya-kristianto.com"
    validation_domain = "aditya-kristianto.com"
  }
  
   validation_option {
    domain_name       = "rust-stock-app.aditya-kristianto.com"
    validation_domain = "aditya-kristianto.com"
  }
  
   validation_option {
    domain_name       = "rust-uuid-app.aditya-kristianto.com"
    validation_domain = "aditya-kristianto.com"
  }
  
  tags = {
    Name = "My SSL Certificate"
  }
}
# ==============================================================================

resource "aws_route53_record" "record" {
  for_each = {
    for dvo in aws_acm_certificate.my_certificate.domain_validation_options : dvo.domain_name => {
      name    = dvo.resource_record_name
      record  = dvo.resource_record_value
      type    = dvo.resource_record_type
    }
  }
  
  allow_overwrite = true
  name            = each.value.name
  records         = [each.value.record]
  ttl             = 60
  type            = each.value.type
  zone_id         = "Z0148722AZOXRNZ0DFO2"  # Use your existing hosted zone ID
}

resource "aws_acm_certificate_validation" "my_cert_validation" {
  certificate_arn         = aws_acm_certificate.my_certificate.arn
  validation_record_fqdns = [for record in aws_route53_record.record : record.fqdn]
}
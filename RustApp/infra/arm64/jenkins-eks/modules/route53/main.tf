# Route53 for Jenkins Master EC2
# ==============================================================================
# resource "aws_route53_record" "jenkins_record" {
#   zone_id  = "Z0148722AZOXRNZ0DFO2"  # Use your existing hosted zone ID
#   name     = "jenkins"
#   type     = "A"
#   ttl      = 300

#   # Set the value to the public IP of the EC2 instance
#   records = [var.jenkins_master_record_value]
# }

# resource "aws_route53_record" "portainer_record" {
#   zone_id  = "Z0148722AZOXRNZ0DFO2"  # Use your existing hosted zone ID
#   name     = "portainer"
#   type     = "A"
#   ttl      = 300

#   # Set the value to the public IP of the EC2 instance
#   records = [var.portainer_record_value]
# }

resource "aws_route53_record" "portainer" {
  zone_id = var.zone_id
  name    = "portainer.aditya-kristianto.com"
  type    = "A"

  alias {
    name                   = var.alb_dns_name
    zone_id                = var.alb_zone_id
    evaluate_target_health = true
  }
}

resource "aws_route53_record" "jenkins" {
  zone_id = var.zone_id
  name    = "jenkins.aditya-kristianto.com"
  type    = "A"

  alias {
    name                   = var.alb_dns_name
    zone_id                = var.alb_zone_id
    evaluate_target_health = true
  }
}
# ==============================================================================
variable "aws_acm_certificate" {
  description = "The value of the AWS ACM Certificate"
  type        = any
}

# variable "jenkins_master_record_value" {
#   description = "The value of the record (e.g., Elastic IP address)"
#   type        = string
# }

# variable "portainer_record_value" {
#   description = "The value of the record (e.g., Elastic IP address)"
#   type        = string
# }

variable "public_ip" {
  description = "The value of the AWS EC2 Public IP"
  type        = string
}

variable "alb_dns_name" {
  description = "The value of the AWS DNS Name"
  type        = string
}

variable "alb_zone_id" {
  description = "The value of the AWS Zone ID"
  type        = string
}

variable "zone_id" {
  description = "The value of the AWS Zone ID"
  type        = string
}
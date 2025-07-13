output "certificate_arn" {
  description = "The ARN of the ACM certificate"
  value       = aws_acm_certificate.my_certificate.arn
}

output "aws_acm_certificate" {
  description = "The ACM Certificate of the AWS ACM certificate"
  value       = aws_acm_certificate.my_certificate
}
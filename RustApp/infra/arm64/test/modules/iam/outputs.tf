output "ssm_profile_name" {
  description = "The IAM Instance Profile Name of The SSM Profile"
  value       = aws_iam_instance_profile.ssm_profile.name
}
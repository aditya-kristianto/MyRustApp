# resource "aws_lb" "portainer_nlb" {
#   name               = "portainer-nlb"
#   internal           = false
#   load_balancer_type = "network"
#   subnets            = var.subnet_ids

#   enable_deletion_protection = false
# }

# resource "aws_lb" "my_alb" {
#   name               = "my-application-load-balancer"
#   internal           = false               # Set to true if for internal use
#   load_balancer_type = "application"
#   security_groups    = var.security_groups
#   subnets            = var.public_subnets
#   enable_deletion_protection = false

#   tags = {
#     Name = "Portainer-Jenkins-ALB"
#   }
# }

# resource "aws_lb_listener" "portainer_https" {
#   load_balancer_arn = aws_lb.portainer_nlb.arn
#   port              = 443
#   protocol          = "TLS"
#   ssl_policy        = "ELBSecurityPolicy-2016-08"  # Specify your desired SSL policy
#   certificate_arn   = var.aws_acm_certificate_arn

#   default_action {
#     type             = "forward"
#     target_group_arn = aws_lb_target_group.portainer.arn
#   }
# }

# resource "aws_lb_listener" "http_listener" {
#   load_balancer_arn = aws_lb.my_alb.arn
#   port              = "80"
#   protocol          = "HTTP"

#   default_action {
#     type = "fixed-response"
#     fixed_response {
#       content_type = "text/plain"
#       message_body = "404 Not Found"
#       status_code  = "404"
#     }
#   }
# }

# resource "aws_lb_listener_rule" "jenkins_rule" {
#   listener_arn = aws_lb_listener.http_listener.arn
#   priority     = 10

#   action {
#     type             = "forward"
#     target_group_arn = aws_lb_target_group.jenkins.arn
#   }

#   condition {
#     host_header {
#       values = ["jenkins.aditya-kristianto.com"]
#     }
#   }
# }

# resource "aws_lb_listener_rule" "portainer_rule" {
#   listener_arn = aws_lb_listener.http_listener.arn
#   priority     = 20

#   action {
#     type             = "forward"
#     target_group_arn = aws_lb_target_group.portainer.arn
#   }

#   condition {
#     host_header {
#       values = ["portainer.aditya-kristianto.com"]
#     }
#   }
# }

# resource "aws_lb_target_group" "portainer" {
#   name     = "nlb-tg"
#   port     = 443   # Use 443 if the backend instance expects HTTPS traffic; use 80 if it’s handling HTTPS-to-HTTP conversion
#   protocol = "TCP" # Use TCP if passing encrypted traffic to the instance directly
#   vpc_id   = var.vpc_id

#   health_check {
#     protocol = "HTTPS"  # Health check over HTTPS
#     path     = "/health" # Adjust this path according to your backend's health check endpoint
#   }
# }

# resource "aws_lb_target_group" "portainer" {
#   name        = "portainer-target-group"
#   port        = 9000                  # Portainer default port
#   protocol    = "HTTP"
#   vpc_id      = var.vpc_id
#   target_type = "instance"            # Or "ip" if you're using IPs directly
# }

# resource "aws_lb_target_group" "jenkins" {
#   name        = "jenkins-target-group"
#   port        = 8080                  # Jenkins default port
#   protocol    = "HTTP"
#   vpc_id      = var.vpc_id
#   target_type = "instance"
# }

resource "aws_eip" "nlb_eip" {
  domain = "vpc"  # Make sure it's in the same VPC
  
  tags = {
    Terraform   = "true"
    Environment = "dev"
  }
}

resource "aws_lb" "nlb" {
  name               = "my-nlb"
  load_balancer_type = "network"
  subnets            = var.public_subnets
  count              = length(var.public_subnets)

  enable_cross_zone_load_balancing = true
  enable_deletion_protection       = false
  enable_http2                     = false

  # Associate Elastic IP
  # dynamic "subnet_mapping" {
  #   for_each = aws_eip.nlb_eip.id != "" ? [aws_eip.nlb_eip.id] : []
  #   content {
  #     subnet_id     = element(var.public_subnets, count.index)
  #     allocation_id = aws_eip.nlb_eip.id
  #   }
  # }
}

resource "aws_lb" "alb" {
  name                       = "my-application-load-balancer"
  internal                   = false               # Set to true if for internal use
  load_balancer_type         = "application"
  security_groups            = var.security_groups
  subnets                    = var.public_subnets
  enable_deletion_protection = false

  tags = {
    Name = "My-ALB"
  }
}

resource "aws_lb_listener" "http_listener" {
  load_balancer_arn = aws_lb.alb.arn
  port              = "80"
  protocol          = "HTTP"

  default_action {
    type = "fixed-response"
    fixed_response {
      content_type = "text/plain"
      message_body = "404 Not Found"
      status_code  = "404"
    }
  }
}

resource "aws_lb_listener_rule" "jenkins_rule" {
  listener_arn = aws_lb_listener.http_listener.arn
  priority     = 10

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.jenkins.arn
  }

  condition {
    host_header {
      values = ["jenkins.aditya-kristianto.com"]
    }
  }
}

resource "aws_lb_listener_rule" "portainer_rule" {
  listener_arn = aws_lb_listener.http_listener.arn
  priority     = 20

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.portainer.arn
  }

  condition {
    host_header {
      values = ["portainer.aditya-kristianto.com"]
    }
  }
}

resource "aws_lb_target_group" "portainer" {
  name        = "portainer-target-group"
  port        = 8000                  # Portainer default port
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "instance"            # Or "ip" if you're using IPs directly
  
  health_check {
    path     = "/"
    protocol = "HTTP"
    interval = 30
    timeout  = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
}

resource "aws_lb_target_group" "jenkins" {
  name        = "jenkins-target-group"
  port        = 8080                  # Jenkins default port
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "instance"
  
  health_check {
    path     = "/"
    protocol = "HTTP"
    interval = 30
    timeout  = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
}
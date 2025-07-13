# Application Load Balancer
# ==============================================================================
resource "aws_lb" "my_alb" {
  name               = "my-application-load-balancer"
  internal           = false               # Set to true if for internal use
  load_balancer_type = "application"
  security_groups    = [aws_security_group.alb_sg.id]
  subnets            = var.public_subnets
  enable_deletion_protection = false
  enable_cross_zone_load_balancing = true

  tags = {
    Name        = "my-alb"
    Environment = "dev"
    Terraform   = "true"
  }
}
# ==============================================================================

# Application Load Balancer Security Group
# ==============================================================================
resource "aws_security_group" "alb_sg" {
  name        = "application_load_balancer_security_group"
  description = "Application Load Balancer Security Group"
  vpc_id      = var.vpc_id
  
  ingress {
    description = "HTTP"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  ingress {
    description = "HTTPS"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  egress {
    description = "HTTP"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
  
  egress {
    description = "HTTP"
    from_port   = 81
    to_port     = 81
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
  
  egress {
    description = "HTTP"
    from_port   = 82
    to_port     = 82
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
  
  egress {
    description = "HTTP"
    from_port   = 83
    to_port     = 83
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
  
  egress {
    description = "HTTP"
    from_port   = 84
    to_port     = 84
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
  
  egress {
    description = "HTTPS"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }
  
  tags = {
    Name        = "my-application-load-balancer-security_group"
    Environment = "dev"
    Terraform   = "true"
  }
}
# ==============================================================================

# Application Load Balancer
# ==============================================================================
resource "aws_lb_listener" "http" {
  load_balancer_arn = aws_lb.my_alb.arn
  port              = 80
  protocol          = "HTTP"
  
  default_action {
    type             = "fixed-response"
    
    fixed_response {
      content_type = "text/plain"
      message_body = "No matching rules found"
      status_code  = "404"
    }
  }
}

resource "aws_lb_listener" "https" {
  load_balancer_arn = aws_lb.my_alb.arn
  port              = 443
  protocol          = "HTTPS"
  
  ssl_policy        = "ELBSecurityPolicy-2016-08"
  certificate_arn   = var.certificate_arn
  
  default_action {
    type             = "fixed-response"
    
    fixed_response {
      content_type = "text/plain"
      message_body = "No matching rules found"
      status_code  = "404"
    }
  }
}

resource "aws_lb_listener_rule" "jenkins_http_rule" {
  listener_arn = aws_lb_listener.http.arn
  priority     = 1

  condition {
    host_header {
      values = ["jenkins.aditya-kristianto.com"]
    }
  }

  # action {
  #   type             = "forward"
  #   target_group_arn = aws_lb_target_group.jenkins_master_http.arn
  # }
  
  action {
    type = "forward"
    
    forward {
      target_group {
        arn    = aws_lb_target_group.jenkins_master_http.arn
        weight = 100
      }
    }
  }
}

resource "aws_lb_listener_rule" "jenkins_https_rule" {
  listener_arn = aws_lb_listener.https.arn
  priority     = 2

  condition {
    host_header {
      values = ["jenkins.aditya-kristianto.com"]
    }
  }

  # action {
  #   type             = "forward"
  #   target_group_arn = aws_lb_target_group.jenkins_master_https.arn
  # }
  
  action {
    type = "forward"
    
    forward {
      target_group {
        arn    = aws_lb_target_group.jenkins_master_https.arn
        weight = 100
      }
    }
  }
}

resource "aws_lb_listener_rule" "portainer_http_rule" {
  listener_arn = aws_lb_listener.http.arn
  priority     = 3

  condition {
    host_header {
      values = ["portainer.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.portainer_http.arn
  }
  
  # action {
  #   type = "redirect"
  #   redirect {
  #     protocol = "HTTPS"
  #     host     = "www.portainer.io"
  #     port     = "443"
  #     status_code = "HTTP_301"
  #   }
  # }
}

resource "aws_lb_listener_rule" "portainer_https_rule" {
  listener_arn = aws_lb_listener.https.arn
  priority     = 4

  condition {
    host_header {
      values = ["portainer.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.portainer_https.arn
  }
  
  # action {
  #   type = "redirect"
  #   redirect {
  #     protocol = "HTTPS"
  #     host     = "www.portainer.io"
  #     port     = "443"
  #     status_code = "HTTP_301"
  #   }
  # }
}

resource "aws_lb_listener_rule" "pgadmin_http_rule" {
  listener_arn = aws_lb_listener.http.arn
  priority     = 5

  condition {
    host_header {
      values = ["pgadmin.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.pgadmin_http.arn
  }
} 

resource "aws_lb_listener_rule" "pgadmin_https_rule" {
  listener_arn = aws_lb_listener.https.arn
  priority     = 6

  condition {
    host_header {
      values = ["pgadmin.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.pgadmin_https.arn
  }
}

resource "aws_lb_listener_rule" "postgres_http_rule" {
  listener_arn = aws_lb_listener.http.arn
  priority     = 7

  condition {
    host_header {
      values = ["postgres.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.postgres_http.arn
  }
}

resource "aws_lb_listener_rule" "rust_app_http_rule" {
  listener_arn = aws_lb_listener.http.arn
  priority     = 8

  condition {
    host_header {
      values = ["rust-app.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.rust_app_http.arn
  }
}

resource "aws_lb_listener_rule" "rust_app_https_rule" {
  listener_arn = aws_lb_listener.https.arn
  priority     = 9
  condition {
    host_header {
      values = ["rust-app.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.rust_app_https.arn
  }
}

resource "aws_lb_listener_rule" "rust_auth_app_http_rule" {
  listener_arn = aws_lb_listener.http.arn
  priority     = 10

  condition {
    host_header {
      values = ["rust-auth-app.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.rust_auth_app_http.arn
  }
}

resource "aws_lb_listener_rule" "rust_auth_app_https_rule" {
  listener_arn = aws_lb_listener.https.arn
  priority     = 11
  condition {
    host_header {
      values = ["rust-auth-app.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.rust_auth_app_https.arn
  }
}

resource "aws_lb_listener_rule" "rust_oauth_app_http_rule" {
  listener_arn = aws_lb_listener.http.arn
  priority     = 12

  condition {
    host_header {
      values = ["rust-oauth-app.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.rust_oauth_app_http.arn
  }
}

resource "aws_lb_listener_rule" "rust_oauth_app_https_rule" {
  listener_arn = aws_lb_listener.https.arn
  priority     = 13
  condition {
    host_header {
      values = ["rust-oauth-app.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.rust_oauth_app_https.arn
  }
}

resource "aws_lb_listener_rule" "rust_stock_app_http_rule" {
  listener_arn = aws_lb_listener.http.arn
  priority     = 14

  condition {
    host_header {
      values = ["rust-stock-app.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.rust_stock_app_http.arn
  }
}

resource "aws_lb_listener_rule" "rust_stock_app_https_rule" {
  listener_arn = aws_lb_listener.https.arn
  priority     = 15
  
  condition {
    host_header {
      values = ["rust-stock-app.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.rust_stock_app_https.arn
  }
}

resource "aws_lb_listener_rule" "rust_uuid_app_http_rule" {
  listener_arn = aws_lb_listener.http.arn
  priority     = 16

  condition {
    host_header {
      values = ["rust-uuid-app.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.rust_uuid_app_http.arn
  }
}

resource "aws_lb_listener_rule" "rust_uuid_app_https_rule" {
  listener_arn = aws_lb_listener.https.arn
  priority     = 17
  condition {
    host_header {
      values = ["rust-uuid-app.aditya-kristianto.com"]
    }
  }

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.rust_uuid_app_https.arn
  }
}
# ==============================================================================

# Load Balancer Target Group for Jenkins Master
# ==============================================================================
resource "aws_lb_target_group" "jenkins_master_http" {
  name        = "jenkins-master-http-tg"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/"
    protocol            = "HTTP"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "jenkins-master-http-tg"
    Purpose = "Forward from jenkins.aditya-kristianto.com"
  }
}

resource "aws_lb_target_group" "jenkins_master_https" {
  name        = "jenkins-master-https-tg"
  port        = 443
  protocol    = "HTTPS"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/"
    protocol            = "HTTPS"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "jenkins-master-https-tg"
    Purpose = "Forward from jenkins.aditya-kristianto.com"
  }
}
# ==============================================================================

# Load Balancer Target Group for Portainer
# ==============================================================================
resource "aws_lb_target_group" "portainer_http" {
  name        = "portainer-http-tg"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/"
    protocol            = "HTTP"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "portainer-http-tg"
    Purpose = "Forward from portainer.aditya-kristianto.com"
  }
}

resource "aws_lb_target_group" "portainer_https" {
  name        = "portainer-https-tg"
  port        = 443
  protocol    = "HTTPS"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/"
    protocol            = "HTTPS"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "portainer-https-tg"
    Purpose = "Forward from portainer.aditya-kristianto.com"
  }
}
# ==============================================================================

# Load Balancer Target Group for PG Admin
# ==============================================================================
resource "aws_lb_target_group" "pgadmin_http" {
  name        = "pgadmin-http-tg"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/"
    protocol            = "HTTP"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "pgadmin-http-tg"
    Purpose = "Forward from pgadmin.aditya-kristianto.com"
  }
}

resource "aws_lb_target_group" "pgadmin_https" {
  name        = "pgadmin-https-tg"
  port        = 443
  protocol    = "HTTPS"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/"
    protocol            = "HTTPS"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "pgadmin-https-tg"
    Purpose = "Forward from pgadmin.aditya-kristianto.com"
  }
}
# ==============================================================================

# Load Balancer Target Group for PG Admin
# ==============================================================================
resource "aws_lb_target_group" "postgres_http" {
  name        = "postgres-http-tg"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/"
    protocol            = "HTTP"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "postgres-http-tg"
    Purpose = "Forward from postgres.aditya-kristianto.com"
  }
}
# ==============================================================================

# Load Balancer Target Group for Rust App
# ==============================================================================
resource "aws_lb_target_group" "rust_app_http" {
  name        = "rust-app-http-tg"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/v1/healthcheck"
    protocol            = "HTTP"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "rust-app-http-tg"
    Purpose = "Forward from rust-app.aditya-kristianto.com"
  }
}

resource "aws_lb_target_group" "rust_app_https" {
  name        = "rust-app-https-tg"
  port        = 80
  protocol    = "HTTPS"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/v1/healthcheck"
    protocol            = "HTTPS"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "rust-app-https-tg"
    Purpose = "Forward from rust-app.aditya-kristianto.com"
  }
}
# ==============================================================================

# Load Balancer Target Group for Rust Stock App
# ==============================================================================
resource "aws_lb_target_group" "rust_auth_app_http" {
  name        = "rust-auth-app-http-tg"
  port        = 81
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/v1/healthcheck"
    protocol            = "HTTP"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "rust-auth-app-http-tg"
    Purpose = "Forward from rust-auth-app.aditya-kristianto.com"
  }
}

resource "aws_lb_target_group" "rust_auth_app_https" {
  name        = "rust-auth-app-https-tg"
  port        = 81
  protocol    = "HTTPS"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/v1/healthcheck"
    protocol            = "HTTPS"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "rust-auth-app-https-tg"
    Purpose = "Forward from rust-auth-app.aditya-kristianto.com"
  }
}
# ==============================================================================

# Load Balancer Target Group for Rust OAuth App
# ==============================================================================
resource "aws_lb_target_group" "rust_oauth_app_http" {
  name        = "rust-oauth-app-http-tg"
  port        = 82
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/v1/healthcheck"
    protocol            = "HTTP"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "rust-oauth-app-http-tg"
    Purpose = "Forward from rust-oauth-app.aditya-kristianto.com"
  }
}

resource "aws_lb_target_group" "rust_oauth_app_https" {
  name        = "rust-oauth-app-https-tg"
  port        = 82
  protocol    = "HTTPS"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/v1/healthcheck"
    protocol            = "HTTPS"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "rust-oauth-app-https-tg"
    Purpose = "Forward from rust-oauth-app.aditya-kristianto.com"
  }
}
# ==============================================================================

# Load Balancer Target Group for Rust Stock App
# ==============================================================================
resource "aws_lb_target_group" "rust_stock_app_http" {
  name        = "rust-stock-app-http-tg"
  port        = 83
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/v1/healthcheck"
    protocol            = "HTTP"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "rust-stock-app-http-tg"
    Purpose = "Forward from rust-stock-app.aditya-kristianto.com"
  }
}

resource "aws_lb_target_group" "rust_stock_app_https" {
  name        = "rust-stock-app-https-tg"
  port        = 83
  protocol    = "HTTPS"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/v1/healthcheck"
    protocol            = "HTTPS"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "rust-stock-app-https-tg"
    Purpose = "Forward from rust-stock-app.aditya-kristianto.com"
  }
}
# ==============================================================================

# Load Balancer Target Group for Rust UUID App
# ==============================================================================
resource "aws_lb_target_group" "rust_uuid_app_http" {
  name        = "rust-uuid-app-http-tg"
  port        = 84
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/v1/healthcheck"
    protocol            = "HTTP"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "rust-uuid-app-http-tg"
    Purpose = "Forward from rust-uuid-app.aditya-kristianto.com"
  }
}

resource "aws_lb_target_group" "rust_uuid_app_https" {
  name        = "rust-uuid-app-https-tg"
  port        = 84
  protocol    = "HTTPS"
  vpc_id      = var.vpc_id
  target_type = "ip"
  
  stickiness {
    type            = "lb_cookie"
    enabled         = true
  }

  deregistration_delay = 30

  health_check {
    path                = "/v1/healthcheck"
    protocol            = "HTTPS"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 3
    unhealthy_threshold = 2
  }
  
  tags = {
    Name    = "rust-uuid-app-https-tg"
    Purpose = "Forward from rust-uuid-app.aditya-kristianto.com"
  }
}
# ==============================================================================

# Route53 Record for Jenkins Master
# ==============================================================================
resource "aws_route53_record" "jenkins_master" {
  zone_id = "Z0148722AZOXRNZ0DFO2"
  name    = "jenkins.aditya-kristianto.com"
  type    = "A"

  alias {
    name                   = aws_lb.my_alb.dns_name
    zone_id                = aws_lb.my_alb.zone_id
    evaluate_target_health = true
  }
}
# ==============================================================================

# Route53 Record for PGAdmin
# ==============================================================================
resource "aws_route53_record" "pgadmin" {
  zone_id = "Z0148722AZOXRNZ0DFO2"
  name    = "pgadmin.aditya-kristianto.com"
  type    = "A"

  alias {
    name                   = aws_lb.my_alb.dns_name
    zone_id                = aws_lb.my_alb.zone_id
    evaluate_target_health = true
  }
}
# ==============================================================================

# Route53 Record for Postgres
# ==============================================================================
resource "aws_route53_record" "postgres" {
  zone_id = "Z0148722AZOXRNZ0DFO2"
  name    = "postgres.aditya-kristianto.com"
  type    = "A"

  alias {
    name                   = aws_lb.my_alb.dns_name
    zone_id                = aws_lb.my_alb.zone_id
    evaluate_target_health = true
  }
}
# ==============================================================================

# Route53 Record for Rust App
# ==============================================================================
resource "aws_route53_record" "rust_app" {
  zone_id = "Z0148722AZOXRNZ0DFO2"
  name    = "rust-app.aditya-kristianto.com"
  type    = "A"

  alias {
    name                   = aws_lb.my_alb.dns_name
    zone_id                = aws_lb.my_alb.zone_id
    evaluate_target_health = true
  }
}
# ==============================================================================

# Route53 Record for Rust Auth App
# ==============================================================================
resource "aws_route53_record" "rust_auth_app" {
  zone_id = "Z0148722AZOXRNZ0DFO2"
  name    = "rust-auth-app.aditya-kristianto.com"
  type    = "A"

  alias {
    name                   = aws_lb.my_alb.dns_name
    zone_id                = aws_lb.my_alb.zone_id
    evaluate_target_health = true
  }
}
# ==============================================================================

# Route53 Record for Rust OAuth App
# ==============================================================================
resource "aws_route53_record" "rust_oauth_app" {
  zone_id = "Z0148722AZOXRNZ0DFO2"
  name    = "rust-oauth-app.aditya-kristianto.com"
  type    = "A"

  alias {
    name                   = aws_lb.my_alb.dns_name
    zone_id                = aws_lb.my_alb.zone_id
    evaluate_target_health = true
  }
}
# ==============================================================================

# Route53 Record for Rust Stock App
# ==============================================================================
resource "aws_route53_record" "rust_stock_app" {
  zone_id = "Z0148722AZOXRNZ0DFO2"
  name    = "rust-stock-app.aditya-kristianto.com"
  type    = "A"

  alias {
    name                   = aws_lb.my_alb.dns_name
    zone_id                = aws_lb.my_alb.zone_id
    evaluate_target_health = true
  }
}
# ==============================================================================

# Route53 Record for Rust UUID App
# ==============================================================================
resource "aws_route53_record" "rust_uuid_app" {
  zone_id = "Z0148722AZOXRNZ0DFO2"
  name    = "rust-uuid-app.aditya-kristianto.com"
  type    = "A"

  alias {
    name                   = aws_lb.my_alb.dns_name
    zone_id                = aws_lb.my_alb.zone_id
    evaluate_target_health = true
  }
}
# ==============================================================================

resource "aws_lb_target_group_attachment" "jenkins_master_http" {
  target_group_arn = aws_lb_target_group.jenkins_master_http.arn
  target_id        = var.jenkins_master_private_ip
  port             = 80
}

resource "aws_lb_target_group_attachment" "jenkins_master_https" {
  target_group_arn = aws_lb_target_group.jenkins_master_https.arn
  target_id        = var.jenkins_master_private_ip
  port             = 443
}

resource "aws_lb_target_group_attachment" "portainer_http" {
  target_group_arn = aws_lb_target_group.portainer_http.arn
  target_id        = var.portainer_private_ip
  port             = 80
}

resource "aws_lb_target_group_attachment" "portainer_https" {
  target_group_arn = aws_lb_target_group.portainer_https.arn
  target_id        = var.portainer_private_ip
  port             = 443
}

resource "aws_lb_target_group_attachment" "pgadmin_http" {
  target_group_arn = aws_lb_target_group.pgadmin_http.arn
  target_id        = var.pgadmin_private_ip
  port             = 80
}

resource "aws_lb_target_group_attachment" "pgadmin_https" {
  target_group_arn = aws_lb_target_group.pgadmin_https.arn
  target_id        = var.pgadmin_private_ip
  port             = 443
}

resource "aws_lb_target_group_attachment" "postgres_http" {
  target_group_arn = aws_lb_target_group.postgres_http.arn
  target_id        = var.pgadmin_private_ip
  port             = 80
}

resource "aws_lb_target_group_attachment" "rust_app_http" {
  target_group_arn = aws_lb_target_group.rust_app_http.arn
  target_id        = var.rust_app_private_ip
  port             = 80
}

resource "aws_lb_target_group_attachment" "rust_app_https" {
  target_group_arn = aws_lb_target_group.rust_app_https.arn
  target_id        = var.rust_app_private_ip
  port             = 443
}

resource "aws_lb_target_group_attachment" "rust_auth_app_http" {
  target_group_arn = aws_lb_target_group.rust_auth_app_http.arn
  target_id        = var.rust_app_private_ip
  port             = 81
}

resource "aws_lb_target_group_attachment" "rust_auth_app_https" {
  target_group_arn = aws_lb_target_group.rust_auth_app_https.arn
  target_id        = var.rust_app_private_ip
  port             = 444
}

resource "aws_lb_target_group_attachment" "rust_oauth_app_http" {
  target_group_arn = aws_lb_target_group.rust_oauth_app_http.arn
  target_id        = var.rust_app_private_ip
  port             = 82
}

resource "aws_lb_target_group_attachment" "rust_oauth_app_https" {
  target_group_arn = aws_lb_target_group.rust_oauth_app_https.arn
  target_id        = var.rust_app_private_ip
  port             = 445
}

resource "aws_lb_target_group_attachment" "rust_stock_app_http" {
  target_group_arn = aws_lb_target_group.rust_stock_app_http.arn
  target_id        = var.rust_app_private_ip
  port             = 83
}

resource "aws_lb_target_group_attachment" "rust_stock_app_https" {
  target_group_arn = aws_lb_target_group.rust_stock_app_https.arn
  target_id        = var.rust_app_private_ip
  port             = 446
}

resource "aws_lb_target_group_attachment" "rust_uuid_app_http" {
  target_group_arn = aws_lb_target_group.rust_uuid_app_http.arn
  target_id        = var.rust_app_private_ip
  port             = 84
}

resource "aws_lb_target_group_attachment" "rust_uuid_app_https" {
  target_group_arn = aws_lb_target_group.rust_uuid_app_https.arn
  target_id        = var.rust_app_private_ip
  port             = 447
}

# Route53 Record for Portainer
# ==============================================================================
resource "aws_route53_record" "portainer" {
  zone_id = "Z0148722AZOXRNZ0DFO2"
  name    = "portainer.aditya-kristianto.com"
  type    = "A"

  alias {
    name                   = aws_lb.my_alb.dns_name
    zone_id                = aws_lb.my_alb.zone_id
    evaluate_target_health = true
  }
}
# ==============================================================================

# resource "aws_lb_target_group_attachment" "portainer_ip_targets" {
#   target_group_arn = aws_lb_target_group.portainer.arn
#   target_id        = var.portainer_private_ip
#   port             = 80
# }

# resource "aws_lb_target_group_attachment" "jenkins_master_ip_targets" {
#   target_group_arn = aws_lb_target_group.jenkins_master.arn
#   target_id        = var.jenkins_master_private_ip
#   port             = 80
# }

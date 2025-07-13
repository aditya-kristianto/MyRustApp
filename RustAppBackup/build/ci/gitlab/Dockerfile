FROM centos:latest

# RUN yum install -y curl policycoreutils-python 
RUN yum install -y openssh-server 
RUN yum install -y openssh-clients 
RUN yum install -y perl
# Enable OpenSSH server daemon if not enabled: systemctl status sshd
RUN systemctl enable sshd
RUN systemctl start sshd
# Check if opening the firewall is needed with: systemctl status firewalld
RUN firewall-cmd --permanent --add-service=http
RUN firewall-cmd --permanent --add-service=https
RUN systemctl reload firewalld

RUN yum install postfix
RUN systemctl enable postfix
RUN systemctl start postfix

RUN curl https://packages.gitlab.com/install/repositories/gitlab/gitlab-ee/script.rpm.sh | bash

RUN EXTERNAL_URL="https://gitlab.example.com" yum install -y gitlab-ee
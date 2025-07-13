# Use official Jenkins LTS image as the base image
# FROM jenkins/jenkins:alpine3.19-jdk21
FROM jenkins/jenkins:2.452.1-jdk21

# Set the working directory (if needed)
WORKDIR /home/jenkins

# Switch to root user for installation
USER root

# Install Docker dependencies and Docker
# RUN apk update \
    # && apk add --no-cache \
    # ca-certificates \
    # curl \
    # gnupg \
    # linux-headers \
    # openssl \
    # docker \
    # docker-cli \
    # fuse-overlayfs

# Install dependencies and tools for adding the Docker repository
RUN apt-get update && \
    apt-get install -y \
        ca-certificates \
        curl \
        gnupg \
        lsb-release \
        openssl \
        fuse-overlayfs \
        && apt-get clean

# Add Docker’s official GPG key
RUN curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

# Set up the Docker stable repository
RUN echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" > /etc/apt/sources.list.d/docker.list

# Install Docker Engine and CLI
RUN apt-get update && \
    apt-get install -y \
        docker-ce \
        docker-ce-cli \
        containerd.io \
        # linux-headers-$(uname -r) \
        && apt-get clean

# Install Docker Compose (optional)
# RUN apk add --no-cache docker-compose

# Create Jenkins user
# RUN adduser -D jenkins

# Add Jenkins user to the Docker group
# RUN addgroup jenkins docker

# Add jenkins user to the sudo group
RUN usermod -aG sudo jenkins

# Expose Docker socket to Jenkins
# VOLUME /var/run/docker.sock

# Set Docker daemon socket permissions (optional)
# RUN chown root:docker /var/run/docker.sock && \
#     chmod 660 /var/run/docker.sock
RUN mkdir -p /var/jenkins_home/logs/slaves/Docker\ Agent
RUN chown -R jenkins:jenkins /var/jenkins_home/

COPY build/ci/jenkins/resolv.conf /etc/resolv.conf

# Copy the Docker daemon configuration file
# COPY build/ci/jenkins/daemon.json /etc/docker/daemon.json

# Start Docker daemon with custom configuration
# CMD ["dockerd"]
# CMD ["dockerd", "--config-file", "/etc/docker/daemon.json"]

# Set Docker socket environment variable
# ENV DOCKER_HOST=unix:///var/run/docker.sock

# Switch back to the Jenkins user
# USER jenkins


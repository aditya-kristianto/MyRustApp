# Use a base image with the necessary tools (e.g., with Maven and Git)
FROM maven:3.9.6-sapmachine-21

# Install additional tools or dependencies if needed
RUN apt-get update && \
    apt-get install -y git

# Set up a non-root user (optional but recommended for security)
RUN groupadd -r jenkins && useradd -r -g jenkins jenkins

# Switch to the non-root user
USER jenkins

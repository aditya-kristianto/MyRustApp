# Use a minimal base image
FROM public.ecr.aws/nginx/nginx:1.25-alpine-slim

ARG TEMPLATE_TYPE
ARG ASSET_NAME
ARG ASSET_VERSION

# Remove the default Nginx configuration
RUN rm /etc/nginx/conf.d/default.conf

# Copy your custom Nginx configuration file
COPY build/ci/rust-asset/nginx.conf /etc/nginx/conf.d/

# Clean the directory
RUN rm -rf /usr/share/nginx/html

# Create nested directories
RUN mkdir -p /usr/share/nginx/html/assets

# Copy your public assets into the container
COPY static/$ASSET_NAME/$ASSET_VERSION/$TEMPLATE_TYPE/src /usr/share/nginx/html/assets

# Remove .DS_Store files if they exist
RUN find /usr/share/nginx/html/assets -type f -name .DS_Store -exec rm {} \;

# Define the working directory
WORKDIR /usr/share/nginx/html

# Expose a port if your assets require serving through a web server
EXPOSE 80/tcp

# Start the Nginx web server when the container starts
CMD ["nginx", "-g", "daemon off;"]
#!/bin/bash

# Check if the first argument is empty
if [ -z "$1" ]; then
    echo "The first argument is empty."
    exit 1
fi

if [ -z "$2" ]; then
    echo "The second argument is empty."
    exit 1
fi

if [ -z "$3" ]; then
    echo "The third argument is empty."
    exit 1
fi

if [ -z "$4" ]; then
    echo "The fourth argument is empty."
    exit 1
fi

# Define the name of the Docker container
AWS_ACCOUNT_ID="${1}"
AWS_REGION="${2}"
IMAGE_NAME="${3}"
IMAGE_TAG="${4}"

latest_image=$(docker images \
        --format "{{.Repository}}:{{.Tag}}" | grep "^${IMAGE_NAME}:" | head -n 1)

if [ -z "${latest_image}" ]; then
    echo "Docker Image does not exist."
    exit 1
fi

docker tag ${latest_image} ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${latest_image}

aws ecr get-login-password --region ${AWS_REGION} | docker login --username AWS --password-stdin ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${IMAGE_NAME}

# Check if the ECR repository exists
if aws ecr describe-repositories --region ${AWS_REGION} --repository-names ${IMAGE_NAME} > /dev/null 2>&1; then
    echo "ECR repository '${IMAGE_NAME}' already exists.";
else
    echo "Creating ECR repository '${IMAGE_NAME}'...";
    aws ecr create-repository --region ${AWS_REGION} --repository-name ${IMAGE_NAME};
    echo "ECR repository '${IMAGE_NAME}' created successfully.";
fi

# Check if the Docker image exists
if docker images "${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${latest_image}" | grep -q "${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${IMAGE_NAME}"; then
    echo "Docker image ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${latest_image} exists locally."
    docker push --compress ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${latest_image}
else
    echo "${RED}Docker image ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${latest_image} does not exist locally.${NC}"
fi
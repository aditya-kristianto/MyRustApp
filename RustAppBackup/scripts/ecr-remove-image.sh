#!/bin/bash

# Check if the first argument is empty
if [ -z "$1" ]; then
    echo "The first argument is empty.";
fi

if [ -z "$2" ]; then
    echo "The second argument is empty."; exit 1;
fi

if [ -z "$3" ]; then
    echo "The third argument is empty."; exit 1;
fi

ARCH=$(uname -m)
PLATFORM=""
if [ "$ARCH" == "x86_64" ]; then
  echo "This is linux/amd64."
  PLATFORM="linux/amd64"
elif [ "$ARCH" == "aarch64" ]; then
  echo "This is linux/arm64."
  PLATFORM="linux/arm64"
else
  echo "This is another architecture: $ARCH"
fi

# Set your AWS region and ECR repository name
aws_region="${1}"
ecr_repository_name="${2}"

# Define the number of versions to keep
versions_to_keep="${3}"

# Get a list of image digest values for the repository
image_digests=$(aws ecr describe-images --repository-name "${ARCH}/$ecr_repository_name" --region "$aws_region" --query 'imageDetails' --output json)

# Sort the image digests in reverse order to identify the oldest images
sorted_digests=$(echo "$image_digests" | jq -r 'sort_by(.imagePushedAt) | reverse')

digest_length=$(echo $sorted_digests | jq 'length')

# Remove images except for the latest N (versions_to_keep) images
if [ $digest_length > 0 ]; then
    for ((i=versions_to_keep; i<$digest_length; i++)); do
      image_digest=$(echo $sorted_digests | jq ".[${i}].imageDigest")
    
      echo "Deleting image with digest: $image_digest"
      aws ecr batch-delete-image --repository-name "${ARCH}/$ecr_repository_name" --region "$aws_region" --image-ids imageDigest="$image_digest"
    done
fi
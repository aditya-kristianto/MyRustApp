#!/bin/bash

# Set your AWS region and ECR repository name
aws_region="ap-southeast-1"
ecr_repository_name="auth"

# Use the AWS CLI to list image tags in the ECR repository
image_tags=$(aws ecr list-images --repository-name "$ecr_repository_name" --region "$aws_region" --query 'imageIds[].imageTag' --output json)

# Check if there are any image tags
if [[ ! -z "$image_tags" ]]; then
    # Use jq to filter out null values and sort the non-null image tags
    sorted_tags=$(echo "$image_tags" | jq -r 'map(select(. != null)) | sort')

    # Check if there are non-null tags
    if [[ ! -z "$sorted_tags" ]]; then
        # Extract the last (latest) non-null version
        latest_version=$(echo "$sorted_tags" | jq -r 'last')

        if [[ -z "$latest_version" ]]; then
          echo "Latest image tag version: $latest_version"
        else
          echo "$latest_version : " $latest_version
        fi
    else
        echo "No non-null image tags found in the ECR repository."
    fi
else
    echo "No image tags found in the ECR repository."
fi

#!/bin/bash

# Check if the first argument is empty
if [ -z "$1" ]; then
    echo "The first argument is empty.";
fi

# Check if the second argument is empty
if [ -z "$2" ]; then
    echo "The second argument is empty.";
fi

# Define the pattern to match Docker image names
IMAGE_NAME_PATTERN="${1}"

# Define the number of versions to keep
VERSIONS_TO_KEEP="${2}"

# Get the IDs of the oldest 5 Docker images matching the pattern
IMAGE_IDS=$(docker image ls --format '{{.Repository}}:{{.Tag}}' --filter "dangling=false" | grep "$IMAGE_NAME_PATTERN" | awk -F '[:]' '{print $2,$1}' | sort -rV | awk '{print $2":"$1}')

# Count the number of elements in IMAGE_IDS
NUM_IMAGES=$(echo "$IMAGE_IDS" | wc -l)

# Loop through each image ID and remove the corresponding Docker image
counter=1
for ID in $IMAGE_IDS; do
    if [ "$counter" -gt $VERSIONS_TO_KEEP ]; then
        echo "Removing Docker image with ID: $ID"
        docker image rm $ID
    fi
    ((counter++))
done

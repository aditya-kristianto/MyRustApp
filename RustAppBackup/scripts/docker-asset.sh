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

if [ -z "$5" ]; then
    echo "The fifth argument is empty."
    exit 1
fi

if [ -z "$6" ]; then
    echo "The sixth argument is empty."
    exit 1
fi

if [ -z "$7" ]; then
    echo "The seventh argument is empty."
    exit 1
fi

if [ -z "${8}" ]; then
    echo "The eight argument is empty."
fi

if [ -z "${9}" ]; then
    echo "The ninth argument is empty."
fi

if [ -z "${10}" ]; then
    echo "The tenth argument is empty."
fi

if [ -z "${11}" ]; then
    echo "The eleventh argument is empty."
fi

# Define the Docker image name and tag
build_dir_name="${1}"
IMAGE_NAME="${2}"
host_port="${3}"
CONTAINER_HTTP_PORT="${4}"
CONTAINER_TCP_PORT="${5}"
current_version=""
image_version="${6}"
next_version=""
target="${7}"
TEMPLATE_TYPE="${8}"
is_ecr="${9}"
AWS_REGION="${10}"
AWS_ACCOUNT_ID="${11}"

ARCH=$(uname -m)
PLATFORM=""
if [ "$ARCH" == "x86_64" ]; then
  echo "This is linux/amd64."
  PLATFORM="linux/amd64"
elif [ "$ARCH" == "aarch64" ] || [ "$ARCH" == "arm64" ]; then
  echo "This is linux/arm64."
  PLATFORM="linux/arm64"
else
  echo "This is another architecture: $ARCH"
fi

if [ -n "${is_ecr}" ] && [ "${is_ecr}" == "true" ]; then
    if aws ecr describe-repositories --region ${AWS_REGION} --repository-names ${ARCH}/${IMAGE_NAME} >/dev/null 2>&1; then
        echo "ECR repository '${ARCH}/${IMAGE_NAME}' already exists."
    else
        echo "Creating ECR repository '${ARCH}/${IMAGE_NAME}'..."
        
        aws ecr create-repository --region ${AWS_REGION} --repository-name ${ARCH}/${IMAGE_NAME}
        
        echo "ECR repository '${ARCH}/${IMAGE_NAME}' created successfully."
    fi
    
    latest_image_version=$(aws ecr describe-images --repository-name "${ARCH}/${IMAGE_NAME}" --region "$AWS_REGION" --query 'imageDetails[].imageTags' --output json | jq -r '.[] | select(. != null)  | .[]' | sort -t '.' -k 1,1n -k 2,2n -k 3,3n -k 4,4n | tail -n 1)
    
    if [ -n "$latest_image_version" ]; then
        latest_image=${ARCH}/${IMAGE_NAME}:${latest_image_version}
        latest_image=$(echo "$latest_image" | sed 's/""//g; s/""//g; s/"//g; s/ //g')
    fi
else
    # Get the latest Docker image (based on creation date)
    latest_image=$(docker images --format "{{.Repository}}:{{.Tag}}" | grep "^${ARCH}/${IMAGE_NAME}:" | head -n 1)
fi

# Check if an image was found
if [ -n "$latest_image" ]; then
    echo "Image $latest_image already exists."

    # Split the input string by ':' and '-' and store the result in an array
    IFS=':' read -a parts <<<"$latest_image"

    # Check if there are enough parts (at least 2)
    if [ "${#parts[@]}" -ge 2 ]; then
        # Get the version part (index 1)
        version_part="${parts[1]}"

        # Print the version part
        echo "Version number: $version_part"

        # Split the image version into components using a hyphen as a delimiter
        IFS="-" read -ra version_components <<<"$version_part"

        # Split the first part (version) into major, minor, and patch using dot as delimiter
        IFS="." read -ra version_parts <<<"${version_components[0]}"

        if [ "${#version_parts[@]}" -eq 3 ]; then
            major_version="${version_parts[0]}"
            minor_version="${version_parts[1]}"
            patch_version="${version_parts[2]}"

            current_version="${major_version}.${minor_version}.${patch_version}"
        else
            echo "Invalid version format"
            exit 1
        fi

        # Extract the suffix (if present)
        suffix="${version_components[1]}"

        # Increment the patch version
        patch_version=$((patch_version + 1))

        # Check if the patch version has rolled over to 100
        if [ "$patch_version" -eq 100 ]; then
            # Reset the patch version to 0
            patch_version=0

            # Increment the minor version
            minor_version=$((minor_version + 1))

            # Check if the minor version has rolled over to 100
            if [ "$minor_version" -eq 100 ]; then
                # Reset the minor version to 0
                minor_version=0

                # Increment the major version
                major_version=$((major_version + 1))
            fi
        fi

        # Reconstruct the version string
        next_version="$major_version.$minor_version.$patch_version"

        # Extract the suffix (if present)
        suffix="${version_components[1]}"
    else
        echo "Invalid format: Input does not contain ':' and version number"
    fi
else
    echo "Image ${ARCH}/${IMAGE_NAME} does not exist."

    next_version=${image_version}
fi

echo "Current Version: $current_version"
echo "Next Version: $next_version"
echo "Suffix: $target"

# Build the new Docker image (replace with your build command)
docker build \
    --build-arg TEMPLATE_TYPE=${TEMPLATE_TYPE} \
    --no-cache \
    -f build/ci/${build_dir_name}/Dockerfile \
    -t ${ARCH}/${IMAGE_NAME}:${next_version}-${target} .

if [ -n "${is_ecr}" ] && [ "${is_ecr}" == "true" ]; then
    docker tag ${ARCH}/${IMAGE_NAME}:${next_version}-${target} ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ARCH}/${IMAGE_NAME}:${next_version}-${target}
    docker push ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ARCH}/${IMAGE_NAME}:${next_version}-${target}
fi

if [ -n "${is_ecr}" ] && [ "${is_ecr}" == "false" ]; then
    # Stop and remove the old container if it exists
    if docker ps -a --format '{{.Names}}' | grep -Eq "^${IMAGE_NAME}$"; then
        echo "Stopping and removing old container: ${IMAGE_NAME}"
        docker stop ${ARCH}/${IMAGE_NAME}
        docker rm ${ARCH}/${IMAGE_NAME}
    fi

    # Start a new container with the same name and updated image
    docker run --name "${ARCH}/${IMAGE_NAME}" -d -p ${host_port}:${container_port} ${ARCH}/${IMAGE_NAME}:${next_version}-${target}

    # Display container statu
    docker ps -a --filter "name=${ARCH}/${IMAGE_NAME}"
fi

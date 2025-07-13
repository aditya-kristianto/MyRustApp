#!/bin/bash
export DOCKER_BUILDKIT=1

# Check if the first argument is empty
if [ -z "${1}" ]; then
    echo "The first argument is empty."
    exit 1
fi

if [ -z "${2}" ]; then
    echo "The second argument is empty."
    exit 1
fi

if [ -z "${3}" ]; then
    echo "The third argument is empty."
    exit 1
fi

if [ -z "${4}" ] && [ -n "${is_base_image}" ] && [ "${is_base_image}" == "true" ]; then
    echo "The fourth argument is empty."
fi

if [ -z "${5}" ]; then
    echo "The fifth argument is empty."
    exit 1
fi

if [ -z "${6}" ]; then
    echo "The sixth argument is empty."
    exit 1
fi

if [ -z "${7}" ]; then
    echo "The seventh argument is empty."
    exit 1
fi

if [ -z "${8}" ] && [ -n "${is_base_image}" ] && [ "${is_base_image}" == "true" ]; then
    echo "The eighth argument is empty."
    exit 1
fi

if [ -z "${9}" ] && [ -n "${is_base_image}" ] && [ "${is_base_image}" == "true" ]; then
    echo "The ninth argument is empty."
    exit 1
fi

if [ -z "${10}" ]; then
    echo "The tenth argument is empty."
    exit 1
fi

if [ -z "${11}" ]; then
    echo "The eleventh argument is empty."
    exit 1
fi

if [ -z "${12}" ]; then
    echo "The twelfth argument is empty."
fi

if [ -z "${13}" ]; then
    echo "The thirteenth argument is empty."
fi

if [ -z "${14}" ]; then
    echo "The fourteenth argument is empty."
fi

if [ -z "${15}" ]; then
    echo "The fifteenth argument is empty."
fi

if [ -z "${16}" ]; then
    echo "The sixteenth argument is empty."
fi

# Define the Docker image name and tag
AWS_ACCOUNT_ID="${1}"
AWS_REGION="${2}"
is_build_multiplatform="${3}"
bin="${4}"
app_name="${5}"
image_name="${6}"
repository_name="rust-alpine"
container_name="${7}"
host_port="${8}"
tcp_port="${9}"
current_version=""
next_version=""
target="${10}"
template_type="${11}"
is_asset_image="${12}"
is_base_image="${13}"
verbose_output="${14}"
ASSET_NAME="${15}"
ASSET_VERSION="${16}"
PLATFORM="linux/arm64" #linux/amd64,linux/arm/v7,linux/ppc64le,linux/s390x,linux/riscv64,windows/amd64"

if  [ "${is_build_multiplatform}" == "true" ] && ! docker buildx ls | grep -q '^mybuilder'; then
    echo "Builder 'mybuilder' does not exist."
    
    # Create a new Buildx builder using the `docker-container` driver
    docker buildx create --name mybuilder --use
    
    # Ensure the builder is active
    docker buildx inspect --bootstrap
fi


# Check if the repository exists
repo_check=$(aws ecr describe-repositories \
    --repository-names ${repository_name} \
    --region ${AWS_REGION} \
    --query 'repositories[*].repositoryName' \
    --output text 2>/dev/null)

# If the repository does not exist, create it
if [ -z "$repo_check" ]; then
    echo "Repository does not exist. Creating repository..."
    aws ecr create-repository \
        --repository-name ${repository_name} \
        --region ${AWS_REGION}
    echo "Repository ${repository_name} created."
else
    echo "Repository ${repository_name} already exists."
fi

rust_alpine_latest_image=$(aws ecr describe-images \
    --repository-name ${repository_name} \
    --region ${AWS_REGION} \
    --filter "tagStatus=TAGGED" \
    --query "imageDetails[?imageTags != null] | sort_by(@, &imagePushedAt)[-1].imageTags[0]" \
    --output text)
    
if [ -z "$rust_alpine_latest_image" ]; then
    echo "ECR Image does not exist. Checking docker image..."
    # Get the latest Docker image (based on creation date)
    rust_alpine_latest_image=$(docker images \
        --format "{{.Repository}}:{{.Tag}}" | grep "^rust-alpine:" | head -n 1)
else
    rust_alpine_latest_image="${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/rust-alpine:${rust_alpine_latest_image}"
fi

# Check if the image exists locally
if [ -n "${is_asset_image}" ] && [ "${is_asset_image}" == "false" ] && [ -z "$rust_alpine_latest_image" ]; then
    if docker image inspect "${rust_alpine_latest_image}" > /dev/null 2>&1; then
        echo "Image ${rust_alpine_latest_image} already exists locally."
    else
        if docker pull "${rust_alpine_latest_image}"; then
            echo "Image ${rust_alpine_latest_image} not found locally. Attempting to pull..."
            echo "Image ${rust_alpine_latest_image} pulled successfully."
        else
            echo "Image ${rust_alpine_latest_image} not found locally. Attempting to build..."
            
            if [ "${is_build_multiplatform}" == "true" ]; then
                docker buildx build \
                    --platform ${PLATFORM} \
                    --build-arg target=${target} \
                    --build-arg verbose_output=${verbose_output} \
                    --no-cache \
                    --push \
                    -f build/ci/rust-alpine/Dockerfile \
                    -t ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/rust-alpine:1.0.0-${target} .
            else
                docker build \
                    --build-arg target=${target} \
                    --build-arg verbose_output=${verbose_output} \
                    --no-cache \
                    -f build/ci/rust-alpine/Dockerfile \
                    -t rust-alpine:1.0.0-${target} .
            fi
            
            echo "Image ${rust_alpine_latest_image} build successfully."
        fi
    fi
    
    if ! docker image inspect "${rust_alpine_latest_image}" > /dev/null 2>&1; then
        echo "Failed to pull or build image ${rust_alpine_latest_image}."
        exit 1
    fi
        
    rust_alpine_latest_image=$(docker images --format "{{.Repository}}:{{.Tag}}" | grep "^rust-alpine:" | head -n 1)
fi
# End - Check if the image exists locally

repository_name="${image_name}"
latest_image=$(aws ecr describe-images \
    --repository-name ${repository_name} \
    --region ${AWS_REGION} \
    --filter "tagStatus=TAGGED" \
    --query "imageDetails[?imageTags != null] | sort_by(@, &imagePushedAt)[-1].imageTags[0]" \
    --output text)

if [ -z "$latest_image" ]; then
    echo "ECR Image does not exist. Checking docker image..."
    # Get the latest Docker image (based on creation date)
    latest_image=$(docker images --format "{{.Repository}}:{{.Tag}}" | grep "^${image_name}:" | head -n 1)
fi

# Check if an image was found
if [ -n "$latest_image" ] && [ "$latest_image" != "None" ]; then
    echo "Image ${latest_image} already exists."

    # Get the version part (index 1)
    version_part="${latest_image}"

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
        echo "Invalid version format "
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
    next_version="${major_version}.${minor_version}.${patch_version}"

    # Extract the suffix (if present)
    suffix="${version_components[1]}"
else
    echo "Image ${image_name} does not exist."

    next_version=1.0.0
fi

echo "Current Version: ${current_version}"
echo "Next Version: ${next_version}"
echo "Suffix: ${target}"

# Check if the repository exists
echo "Checking if ECR repository '${image_name}' exists..."

if aws ecr describe-repositories --repository-names "${image_name}" --region "${AWS_REGION}" > /dev/null 2>&1; then
    echo "ECR repository '${image_name}' already exists."
else
    echo "ECR repository '${image_name}' does not exist. Creating it now..."
    aws ecr create-repository --repository-name "${image_name}" --region "${AWS_REGION}"
    
    if [ $? -eq 0 ]; then
        echo "ECR repository '${image_name}' created successfully."
    else
        echo "Failed to create ECR repository '${image_name}'."
        exit 1
    fi
fi

# Check if the variable is not empty and has the value "true"
if [ -n "${is_asset_image}" ] && [ "${is_asset_image}" == "true" ]; then
    # Build the new Docker image
    if [ "${is_build_multiplatform}" == "true" ]; then
        docker buildx build \
            --platform ${PLATFORM} \
            --build-arg EXPOSE_HTTP_PORT=${http_port} \
            --build-arg EXPOSE_TCP_PORT=${tcp_port} \
            --build-arg TARGET=${target} \
            --build-arg TEMPLATE_TYPE=${template_type} \
            --build-arg ASSET_NAME=${ASSET_NAME} \
            --build-arg ASSET_VERSION=${ASSET_VERSION} \
            --build-arg VERBOSE_OUTPUT=${verbose_output} \
            --no-cache \
            --push \
            -f build/ci/rust-asset/Dockerfile \
            --output type=docker \
            --load \
            -t ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${image_name}:${next_version}-${target} .
    else
        docker build \
            --build-arg EXPOSE_HTTP_PORT=${http_port} \
            --build-arg EXPOSE_TCP_PORT=${tcp_port} \
            --build-arg TARGET=${target} \
            --build-arg TEMPLATE_TYPE=${template_type} \
            --build-arg ASSET_NAME=${ASSET_NAME} \
            --build-arg ASSET_VERSION=${ASSET_VERSION} \
            --build-arg VERBOSE_OUTPUT=${verbose_output} \
            --no-cache \
            -f build/ci/rust-asset/Dockerfile \
            -t ${image_name}:${next_version}-${target} .
    fi
elif [ -n "${is_base_image}" ] && [ "${is_base_image}" == "true" ]; then
    # Build the new Docker image
    if [ "${is_build_multiplatform}" == "true" ]; then
        docker buildx build \
            --platform ${PLATFORM} \
            --build-arg BASE_IMAGE=${rust_alpine_latest_image} \
            --build-arg BIN=${bin} \
            --build-arg EXPOSE_HTTP_PORT=${http_port} \
            --build-arg EXPOSE_TCP_PORT=${tcp_port} \
            --build-arg TARGET=${target} \
            --build-arg TEMPLATE_TYPE=${template_type} \
            --build-arg VERBOSE_OUTPUT=${verbose_output} \
            --no-cache \
            --push \
            -f build/ci/rust-alpine/Dockerfile \
            --output type=docker \
            --load \
            -t ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${image_name}:${next_version}-${target} .
    else
        docker build \
            --build-arg BASE_IMAGE=${rust_alpine_latest_image} \
            --build-arg BIN=${bin} \
            --build-arg EXPOSE_HTTP_PORT=${http_port} \
            --build-arg EXPOSE_TCP_PORT=${tcp_port} \
            --build-arg TARGET=${target} \
            --build-arg TEMPLATE_TYPE=${template_type} \
            --build-arg VERBOSE_OUTPUT=${verbose_output} \
            --no-cache \
            -f build/ci/rust-alpine/Dockerfile \
            -t ${image_name}:${next_version}-${target} .
    fi
else
    # 473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/rust-alpine:1.0.21-release
    aws ecr get-login-password --region ${AWS_REGION} | docker login --username AWS --password-stdin ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/rust-alpine

    # Build the new Docker image
    if [ "${is_build_multiplatform}" == "true" ]; then
        docker buildx build \
            --platform ${PLATFORM} \
            --build-arg BASE_IMAGE=${rust_alpine_latest_image} \
            --build-arg BIN=${bin} \
            --build-arg EXPOSE_TCP_PORT=${http_port} \
            --build-arg EXPOSE_TCP_PORT=${tcp_port} \
            --build-arg TARGET=${target} \
            --build-arg TEMPLATE_TYPE=${template_type} \
            --build-arg ASSET_NAME=${ASSET_NAME} \
            --build-arg ASSET_VERSION=${ASSET_VERSION} \
            --build-arg VERBOSE_OUTPUT=${verbose_output} \
            --build-arg APP_VERSION=${next_version} \
            --no-cache \
            --push \
            -f build/ci/rust-app/Dockerfile \
            --output type=docker \
            --load \
            -t ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${image_name}:${next_version}-${target} .
    else
        docker build \
            --build-arg BASE_IMAGE=${rust_alpine_latest_image} \
            --build-arg BIN=${bin} \
            --build-arg EXPOSE_TCP_PORT=${http_port} \
            --build-arg EXPOSE_TCP_PORT=${tcp_port} \
            --build-arg TARGET=${target} \
            --build-arg TEMPLATE_TYPE=${template_type} \
            --build-arg ASSET_NAME=${ASSET_NAME} \
            --build-arg ASSET_VERSION=${ASSET_VERSION} \
            --build-arg VERBOSE_OUTPUT=${verbose_output} \
            --build-arg APP_VERSION=${next_version} \
            --no-cache \
            -f build/ci/rust-app/Dockerfile \
            -t ${image_name}:${next_version}-${target} .
    fi
fi

# Check if the Docker image exists
if docker inspect "${image_name}:${next_version}-${target}" > /dev/null 2>&1; then
    echo "Image ${image_name}:${next_version}-${target} exists"
elif docker inspect "${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${image_name}:${next_version}-${target}" > /dev/null 2>&1; then
    echo "Image ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${image_name}:${next_version}-${target} exists"
else
    echo "Image ${image_name}:${next_version}-${target} or ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${image_name}:${next_version}-${target} does not exist"
    exit 1
fi
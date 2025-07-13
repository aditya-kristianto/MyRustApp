#!/bin/bash

# Check if the first argument is empty
if [ -z "${1}" ] && [ -z "${15}"]; then
    echo "The first argument is empty."
    exit 1
fi

if [ -z "${2}" ] && [ -z "${15}" ]; then
    echo "The second argument is empty."
    exit 1
fi

if [ -z "${3}" ] && [ -z "${15}" ]; then
    echo "The third argument is empty."
    exit 1
fi

if [ -z "${4}" ] && [ -z "${15}" ]; then
    echo "The fourth argument is empty."
    exit 1
fi

if [ -z "${5}" ] && [ -z "${15}" ]; then
    echo "The fifth argument is empty."
    exit 1
fi

if [ -z "${6}" ] && [ -z "${15}" ]; then
    echo "The sixth argument is empty."
    exit 1
fi

if [ -z "${7}" ] && [ -z "${15}" ]; then
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

if [ -z "${12}" ]; then
    echo "The twelfth argument is empty."
fi

if [ -z "${13}" ]; then
    echo "The thirteenth argument is empty."
fi

if [ -z "${14}" ]; then
    echo "The fourteenth argument is empty."
fi

if [ -z "${16}" ]; then
    echo "The sixteenth argument is empty."
fi

if [ -z "${17}" ]; then
    echo "The seventeenth argument is empty."
fi

# Define the Docker image name and tag
BUILD_DIR_NAME="${1}"
BIN_NAME="${2}"
IMAGE_NAME="${3}"
host_port="${4}"
CONTAINER_HTTP_PORT="${5}"
CONTAINER_TCP_PORT="${6}"
current_version=""
image_version="${7}"
next_version=""
TARGET="${8}"
TEMPLATE_TYPE="${9}"
is_asset_image="${10}"
IS_ECR="${11}"
AWS_REGION="${12}"
AWS_ACCOUNT_ID="${13}"
VERBOSE_OUTPUT="${14}"
BASE_IMAGE=""
RUST_ALPINE_IMAGE_NAME="rust-alpine"
IS_BUILD_RUST_ALPINE="${15}"
ASSET_NAME="${16}"
ASSET_VERSION="${17}"

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

if [ -n "${IS_BUILD_RUST_ALPINE}" ] && [ "${IS_BUILD_RUST_ALPINE}" == "true" ]; then
    echo "Build ${IMAGE_NAME} docker image ..."
    
    if aws ecr describe-repositories --region ${AWS_REGION} --repository-names ${ARCH}/${IMAGE_NAME} >/dev/null 2>&1; then
        echo "ECR repository '${ARCH}/${IMAGE_NAME}' already exists."
    else
        echo "Creating ECR repository '${ARCH}/${IMAGE_NAME}'..."
        
        aws ecr create-repository --region ${AWS_REGION} --repository-name ${ARCH}/${IMAGE_NAME}
        
        echo "ECR repository '${ARCH}/${IMAGE_NAME}' created successfully."
    fi
    
    latest_image=$(aws ecr describe-images --repository-name "${ARCH}/${IMAGE_NAME}" --region "$AWS_REGION" --query 'imageDetails[].imageTags' --output json | jq -r '.[] | select(. != null) | .[]' | sort -t '.' -k 1,1n -k 2,2n -k 3,3n -k 4,4n | head -n 1)
    
    if [ -z "$latest_image" ]; then
        echo "Image ${IMAGE_NAME} does not exist."
        echo "docker build --platform ${PLATFORM} --build-arg target=${TARGET} --build-arg VERBOSE_OUTPUT=${VERBOSE_OUTPUT} --no-cache -f build/ci/${IMAGE_NAME}/Dockerfile -t ${ARCH}/${IMAGE_NAME}:1.0.0-${TARGET} ."
        
        docker build \
            --platform ${PLATFORM} \
            --build-arg target=${TARGET} \
            --build-arg VERBOSE_OUTPUT=${VERBOSE_OUTPUT} \
            --no-cache \
            -f build/ci/${IMAGE_NAME}/Dockerfile \
            -t ${ARCH}/${IMAGE_NAME}:1.0.0-${TARGET} .
        docker tag ${ARCH}/${IMAGE_NAME}:1.0.0-${TARGET} ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ARCH}/${IMAGE_NAME}:1.0.0-${TARGET}
        docker push --compress ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ARCH}/${IMAGE_NAME}:1.0.0-${TARGET}
    else
        latest_image_version=$(aws ecr describe-images --repository-name "${ARCH}/$IMAGE_NAME" --region "$AWS_REGION" --query 'imageDetails[].imageTags' --output json | jq -r '.[] | select(. != null)  | .[]' | sort -t '.' -k 1,1n -k 2,2n -k 3,3n -k 4,4n | tail -n 1)
    
        if [ -n "$latest_image_version" ]; then
            latest_image=${ARCH}/${IMAGE_NAME}:${latest_image_version}
            latest_image=$(echo "$latest_image" | sed 's/""//g; s/""//g; s/"//g; s/ //g')
        
            # Check if an image was found
            if [ -n "$latest_image" ]; then
                IFS=':' read -a parts <<<"${latest_image}"

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
                    next_version="${major_version}.${minor_version}.${patch_version}"
                    
                    # Extract the suffix (if present)
                    suffix="${version_components[1]}"
                    
                    docker build \
                        --platform ${PLATFORM} \
                        --build-arg target=${TARGET} \
                        --build-arg VERBOSE_OUTPUT=${VERBOSE_OUTPUT} \
                        --no-cache \
                        -f build/ci/${IMAGE_NAME}/Dockerfile \
                        -t ${ARCH}/${IMAGE_NAME}:${next_version}-${TARGET} .
                    docker tag ${ARCH}/${IMAGE_NAME}:${next_version}-${TARGET} ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ARCH}/${IMAGE_NAME}:${next_version}-${TARGET}
                    docker push --compress ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ARCH}/${IMAGE_NAME}:${next_version}-${TARGET}
                else
                    echo "Invalid format: Input does not contain ':' and version number"
                fi
            fi
        fi
    fi
    
    exit
fi

if [ -n "${IS_ECR}" ] && [ "${IS_ECR}" == "true" ]; then
    if aws ecr describe-repositories --region ${AWS_REGION} --repository-names ${ARCH}/${RUST_ALPINE_IMAGE_NAME} >/dev/null 2>&1; then
        echo "ECR repository '${ARCH}/${RUST_ALPINE_IMAGE_NAME}' already exists."
    else
        echo "Creating ECR repository '${ARCH}/${RUST_ALPINE_IMAGE_NAME}'..."
        aws ecr create-repository --region ${AWS_REGION} --repository-name ${ARCH}/${RUST_ALPINE_IMAGE_NAME}
        echo "ECR repository '${ARCH}/${RUST_ALPINE_IMAGE_NAME}' created successfully."
    fi
    
    rust_alpine_latest_image=$(aws ecr describe-images --repository-name "${ARCH}/${RUST_ALPINE_IMAGE_NAME}" --region "$AWS_REGION" --query 'imageDetails[].imageTags' --output json | jq -r '.[] | select(. != null) | .[]' | sort -t '.' -k 1,1n -k 2,2n -k 3,3n -k 4,4n | tail -n 1)
    
    if [ -z "$rust_alpine_latest_image" ]; then
        echo "Image rust-alpine does not exist."
        
        docker build \
            --platform ${PLATFORM} \
            --build-arg target=${TARGET} \
            --build-arg VERBOSE_OUTPUT=${VERBOSE_OUTPUT} \
            --no-cache \
            -f build/ci/${RUST_ALPINE_IMAGE_NAME}/Dockerfile \
            -t ${ARCH}/${RUST_ALPINE_IMAGE_NAME}:1.0.0-${TARGET} .
        docker tag ${ARCH}/${RUST_ALPINE_IMAGE_NAME}:1.0.0-${TARGET} ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ARCH}/${RUST_ALPINE_IMAGE_NAME}:1.0.0-${TARGET}
        docker push --compress ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ARCH}/${RUST_ALPINE_IMAGE_NAME}:1.0.0-${TARGET}

        BASE_IMAGE="${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ARCH}/${RUST_ALPINE_IMAGE_NAME}:1.0.0-${TARGET}"
    else
        BASE_IMAGE="${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ARCH}/${RUST_ALPINE_IMAGE_NAME}:${rust_alpine_latest_image}"
    fi
else
    # Get the latest Docker image (based on creation date)
    rust_alpine_latest_image=$(docker images --format "{{.Repository}}:{{.Tag}}" | grep "^rust-alpine:" | head -n 1)

    if [ -z "$rust_alpine_latest_image" ]; then
        echo "Image rust-alpine does not exist."
        
        docker build \
            --platform ${PLATFORM} \
            --build-arg target=${TARGET} \
            --build-arg VERBOSE_OUTPUT=${VERBOSE_OUTPUT} \
            --no-cache \
            -f build/ci/${RUST_ALPINE_IMAGE_NAME}/Dockerfile \
            -t ${ARCH}/${RUST_ALPINE_IMAGE_NAME}:1.0.0-${TARGET} .
        docker tag ${ARCH}/${RUST_ALPINE_IMAGE_NAME}:1.0.0-${TARGET} ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ARCH}/${RUST_ALPINE_IMAGE_NAME}:1.0.0-${TARGET}
        
        BASE_IMAGE="${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${PLATFORM}/${RUST_ALPINE_IMAGE_NAME}:1.0.0-${TARGET}"
    else
        BASE_IMAGE="${rust_alpine_latest_image}"
    fi
fi

echo "BASE IMAGE : ${BASE_IMAGE}"

if [ -n "${IS_ECR}" ] && [ "${IS_ECR}" == "true" ]; then
    latest_image_version=$(aws ecr describe-images --repository-name "${ARCH}/$IMAGE_NAME" --region "$AWS_REGION" --query 'imageDetails[].imageTags' --output json | jq -r '.[] | select(. != null)  | .[]' | sort -t '.' -k 1,1n -k 2,2n -k 3,3n -k 4,4n | tail -n 1)
    
    if [ -n "$latest_image_version" ]; then
        latest_image=${ARCH}/${IMAGE_NAME}:${latest_image_version}
        latest_image=$(echo "$latest_image" | sed 's/""//g; s/""//g; s/"//g; s/ //g')
    fi
else
    # Get the latest Docker image (based on creation date)
    rust_alpine_latest_image=$(docker images --format "{{.Repository}}:{{.Tag}}" | grep "^rust-alpine:" | head -n 1)

    latest_image=$(docker images --format "{{.Repository}}:{{.Tag}}" | grep "^${IMAGE_NAME}:" | head -n 1)
fi

# Check if an image was found
if [ -n "$latest_image" ]; then
    echo "Image ${latest_image} already exists."

    # Split the input string by ':' and '-' and store the result in an array
    IFS=':' read -a parts <<<"${latest_image}"

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
        next_version="${major_version}.${minor_version}.${patch_version}"

        # Extract the suffix (if present)
        suffix="${version_components[1]}"
    else
        echo "Invalid format: Input does not contain ':' and version number"
    fi
else
    echo "Image ${IMAGE_NAME} does not exist."

    next_version=${image_version}
fi

echo "Current Version: ${current_version}"
echo "Next Version: ${next_version}"
echo "Suffix: ${TARGET}"
echo "Is ECR: ${IS_ECR}"

if [ -n "${IS_ECR}" ] && [ "${IS_ECR}" == "true" ]; then    
    if [ -n "${IMAGE_NAME}" ] && [ "${IMAGE_NAME}" == "rust-web" ]; then
        echo "1. docker build --platform ${PLATFORM} --build-arg BASE_IMAGE=${BASE_IMAGE} --build-arg BIN=${BIN_NAME} --build-arg TARGET=${TARGET} --build-arg EXPOSE_HTTP_PORT=${CONTAINER_HTTP_PORT} --build-arg EXPOSE_TCP_PORT=${CONTAINER_TCP_PORT} --build-arg TEMPLATE_TYPE=${TEMPLATE_TYPE} --no-cache -f build/ci/${BUILD_DIR_NAME}/Dockerfile -t ${IMAGE_NAME}:${next_version}-${TARGET} ."
        
        docker build \
            --platform ${PLATFORM} \
            --build-arg BASE_IMAGE=${BASE_IMAGE} \
            --build-arg BIN=${BIN_NAME} \
            --build-arg TARGET=${TARGET} \
            --build-arg EXPOSE_HTTP_PORT=${CONTAINER_HTTP_PORT} \
            --build-arg EXPOSE_TCP_PORT=${CONTAINER_TCP_PORT} \
            --build-arg TEMPLATE_TYPE=${TEMPLATE_TYPE} \
            --build-arg ASSET_NAME=${ASSET_NAME} \
            --build-arg ASSET_VERSION=${ASSET_VERSION} \
            --build-arg VERBOSE_OUTPUT=${VERBOSE_OUTPUT} \
            --no-cache \
            -f build/ci/${BUILD_DIR_NAME}/Dockerfile \
            -t ${ARCH}/${IMAGE_NAME}:${next_version}-${TARGET} .
    else
        echo "2. docker build --platform ${PLATFORM} --build-arg BASE_IMAGE=${BASE_IMAGE} --build-arg BIN=${BIN_NAME} --build-arg TARGET=${TARGET} --build-arg EXPOSE_HTTP_PORT=${CONTAINER_HTTP_PORT} --build-arg EXPOSE_TCP_PORT=${CONTAINER_TCP_PORT} --build-arg TEMPLATE_TYPE=${TEMPLATE_TYPE} --no-cache -f build/ci/${BUILD_DIR_NAME}/Dockerfile -t ${IMAGE_NAME}:${next_version}-${TARGET} ."
        
        docker build \
            --platform ${PLATFORM} \
            --build-arg BASE_IMAGE=${BASE_IMAGE} \
            --build-arg BIN=${BIN_NAME} \
            --build-arg TARGET=${TARGET} \
            --build-arg EXPOSE_HTTP_PORT=${CONTAINER_HTTP_PORT} \
            --build-arg EXPOSE_TCP_PORT=${CONTAINER_TCP_PORT} \
            --build-arg TEMPLATE_TYPE=${TEMPLATE_TYPE} \
            --build-arg VERBOSE_OUTPUT=${VERBOSE_OUTPUT} \
            --no-cache \
            -f build/ci/${BUILD_DIR_NAME}/Dockerfile \
            -t ${ARCH}/${IMAGE_NAME}:${next_version}-${TARGET} .
    fi

    docker tag ${ARCH}/${IMAGE_NAME}:${next_version}-${TARGET} \
        ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ARCH}/${IMAGE_NAME}:${next_version}-${TARGET}
else
    # Check if the variable is not empty and has the value "true"
    if [ -n "${is_asset_image}" ] && [ "${is_asset_image}" == "true" ]; then
        echo "3. docker build --platform ${PLATFORM} --build-arg EXPOSE_HTTP_PORT=${CONTAINER_HTTP_PORT} --build-arg TARGET=${TARGET} --build-arg TEMPLATE_TYPE=${TEMPLATE_TYPE} --no-cache -f build/ci/${BUILD_DIR_NAME}/Dockerfile -t ${ARCH}/${IMAGE_NAME}:${next_version}-${TARGET} ."

        # Build the new Docker image
        docker build \
            --platform ${PLATFORM} \
            --build-arg EXPOSE_HTTP_PORT=${CONTAINER_HTTP_PORT} \
            --build-arg TARGET=${TARGET} \
            --build-arg TEMPLATE_TYPE=${TEMPLATE_TYPE} \
            --no-cache \
            -f build/ci/${BUILD_DIR_NAME}/Dockerfile \
            -t ${ARCH}/${IMAGE_NAME}:${next_version}-${TARGET} .
    else
        echo "4. docker build --platform ${PLATFORM} --build-arg BASE_IMAGE=${BASE_IMAGE} --build-arg BIN=${BIN_NAME} --build-arg EXPOSE_HTTP_PORT=${CONTAINER_HTTP_PORT} --build-arg EXPOSE_TCP_PORT=${CONTAINER_TCP_PORT} --build-arg TARGET=${TARGET} --build-arg TEMPLATE_TYPE=${TEMPLATE_TYPE} --no-cache -f build/ci/${BUILD_DIR_NAME}/Dockerfile -t ${ARCH}/${IMAGE_NAME}:${next_version}-${TARGET} ."
        
        # Build the new Docker image
        docker build \
            --platform ${PLATFORM} \
            --build-arg BASE_IMAGE=${BASE_IMAGE} \
            --build-arg BIN=${BIN_NAME} \
            --build-arg EXPOSE_HTTP_PORT=${CONTAINER_HTTP_PORT} \
            --build-arg EXPOSE_TCP_PORT=${CONTAINER_TCP_PORT} \
            --build-arg TARGET=${TARGET} \
            --build-arg TEMPLATE_TYPE=${TEMPLATE_TYPE} \
            --no-cache \
            -f build/ci/${BUILD_DIR_NAME}/Dockerfile \
            -t ${ARCH}/${IMAGE_NAME}:${next_version}-${TARGET} .
    fi
fi

if aws ecr describe-repositories --region ${AWS_REGION} --repository-names ${ARCH}/${IMAGE_NAME} >/dev/null 2>&1; then
    echo "ECR repository '${ARCH}/${IMAGE_NAME}' already exists."
else
    echo "Creating ECR repository '${ARCH}/${IMAGE_NAME}'..."
    aws ecr create-repository --region ${AWS_REGION} --repository-name ${ARCH}/${IMAGE_NAME}
    echo "ECR repository '${ARCH}/${IMAGE_NAME}' created successfully."
fi

docker push --compress ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ARCH}/${IMAGE_NAME}:${next_version}-${TARGET}

if [ -n "${IS_ECR}" ] && [ "${IS_ECR}" == "false" ]; then
    # Stop and remove the old container if it exists
    if docker ps -a --format '{{.Names}}' | grep -Eq "^${IMAGE_NAME}$"; then
        echo "Stopping and removing old container: ${IMAGE_NAME}"
        docker stop ${IMAGE_NAME}
        docker rm ${IMAGE_NAME}
    fi

    # Start a new container with the same name and updated image
    docker run --name ${IMAGE_NAME} -d -p ${CONTAINER_HTTP_PORT}:${host_port} ${IMAGE_NAME}:${next_version}-${TARGET}

    # Display container status
    docker ps -a --filter "name=${IMAGE_NAME}"
fi

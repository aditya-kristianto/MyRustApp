#!/bin/bash

# Define the image name
PORTAINER_IMAGE_NAME="portainer/portainer-ce"
JENKINS_IMAGE_NAME="jenkins/jenkins"

# Check if the first argument is empty
if [ -z "$1" ]; then
    echo "The first argument is empty."
fi

if [ -n "${IS_START_CONTAINER}" ] && [ "${IS_START_CONTAINER}" == "false" ] && [ -z "${2}" ]; then
    echo "The second argument is empty."
    exit 1
fi

if [ -n "${IS_START_CONTAINER}" ] && [ "${IS_START_CONTAINER}" == "false" ] && [ -z "${3}" ]; then
    echo "The third argument is empty."
    exit 1
fi

if [ -z "${4}" ]; then
    echo "The fourth argument is empty."
    exit 1
fi

# Define the name of the Docker container
IMAGE_NAME="${1}"
CONTAINER_HTTP_PORT="${2}"
CONTAINER_TCP_PORT="${3}"
HTTP_PORT="${4}"
TCP_PORT="${5}"
IS_START_CONTAINER="${6}"
# Define the name of the Docker network
NETWORK_NAME="${7}"
NETWORK_SUBNET="${8}"
NETWORK_GATEWAY="${9}"
CONTAINER_IP="${10}"
AWS_REGION="ap-southeast-1"
AWS_ACCOUNT_ID="473154593366"

# Login into AWS ECR
aws ecr get-login-password --region ap-southeast-1 | docker login --username AWS --password-stdin 473154593366.dkr.ecr.ap-southeast-1.amazonaws.com

# Check if the network exists
if ! docker network inspect "$NETWORK_NAME" >/dev/null 2>&1; then
    # Create the network
    docker network create \
        --subnet=$NETWORK_SUBNET \
        --gateway=$NETWORK_GATEWAY \
        "$NETWORK_NAME"
    echo "Docker network '$NETWORK_NAME' created."
else
    echo "Docker network '$NETWORK_NAME' already exists."
fi

if [ -n "${IS_START_CONTAINER}" ] && [ "${IS_START_CONTAINER}" == "false" ]; then
    # PostgreSQL connection details
    PG_USER="${11}"       # PostgreSQL username
    PG_PASSWORD="${12}"   # PostgreSQL password
    DATABASE_NAME="${13}" # Name of the database to create
    MIGRATION_DIR="${14}"

    # Check if the container exists
    if docker ps -a --format '{{.Names}}' | grep -q "^$IMAGE_NAME$"; then
        echo "Container '$IMAGE_NAME' already exists. Removing..."
        docker rm -f "$IMAGE_NAME"
    fi

    # Get the latest Docker image (based on version)
    LATEST_IMAGE=$(docker images --format "{{.Repository}}:{{.Tag}}" | grep "^$IMAGE_NAME:" | head -n 1)
    
    if [ -n "${LATEST_IMAGE}" ]; then
        # Run your new Docker image
        docker run -d \
            --name $IMAGE_NAME \
            --network $NETWORK_NAME \
            --ip $CONTAINER_IP \
            -p $CONTAINER_HTTP_PORT:$HTTP_PORT \
            -p $CONTAINER_TCP_PORT:$TCP_PORT \
            $LATEST_IMAGE
    fi

    # Get the latest Docker image (based on version)
    LATEST_IMAGE=$(docker images --format "{{.Repository}}:{{.Tag}}" | grep "^${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${IMAGE_NAME}:" | head -n 1)
    
    if [ -n "${LATEST_IMAGE}" ]; then
        # Run your new Docker image
        docker run -d \
            --name $IMAGE_NAME \
            --network $NETWORK_NAME \
            --ip $CONTAINER_IP \
            -p $CONTAINER_HTTP_PORT:$HTTP_PORT \
            -p $CONTAINER_TCP_PORT:$TCP_PORT \
            $LATEST_IMAGE
    else 
        echo "Docker latest image $IMAGE_NAME not found."
        
        LATEST_IMAGE=$(aws ecr describe-images \
            --repository-name ${IMAGE_NAME} \
            --region ${AWS_REGION} \
            --filter "tagStatus=TAGGED" \
            --query "imageDetails[?imageTags != null] | sort_by(@, &imagePushedAt)[-1].imageTags[0]" \
            --output text)
        
        docker pull "${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${IMAGE_NAME}:${LATEST_IMAGE}";
        echo "Image ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${IMAGE_NAME}:${LATEST_IMAGE} pulled successfully."
        
        if [ -n "${IMAGE_NAME}" ] && [ "${IMAGE_NAME}" == "rust-stock" ]; then
            docker run -d \
                --name $IMAGE_NAME \
                --network $NETWORK_NAME \
                --ip $CONTAINER_IP \
                -p $CONTAINER_HTTP_PORT:$HTTP_PORT \
                -p $CONTAINER_TCP_PORT:$TCP_PORT \
                -e APP_WEB_HTTP_PORT="80" \
                -e POSTGRES_DATABASE="rust_app" \
                -e POSTGRES_HOST="172.18.0.2" \
                -e POSTGRES_PORT="5432" \
                -e POSTGRES_USERNAME="aditya.kristianto" \
                -e POSTGRES_PASSWORD="my secret password" \
                -e POSTGRES_URI="postgres://aditya.kristianto:my secret password@172.18.0.2:5432/rust_app" \
                ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${IMAGE_NAME}:$LATEST_IMAGE
        fi
    fi

    if [ -n "${PG_USER}" ] && [ -n "${PG_PASSWORD}" ] && [ -n "${DATABASE_NAME}" ]; then
        # Check if the database already exists
        docker exec -it postgres bash -c "if ! psql -U "$PG_USER" -d "postgres" -lqt | cut -d \| -f 1 | grep -qw \"$DATABASE_NAME\"; then psql -U \"$PG_USER\" -d \"postgres\" -c \"CREATE DATABASE $DATABASE_NAME\"; fi"

        diesel migration run \
            --migration-dir=$MIGRATION_DIR \
            --database-url "postgres://aditya.kristianto:mysecretpassword@localhost:5432/$DATABASE_NAME"
    fi
else
    DOCKER_IMAGE_NAME=""
    DOCKER_CONTAINER_NAME=""

    if [ -n "${IMAGE_NAME}" ] && [ "${IMAGE_NAME}" == "portainer" ]; then
        DOCKER_IMAGE_NAME=$PORTAINER_IMAGE_NAME
    elif [ -n "${IMAGE_NAME}" ] && [ "${IMAGE_NAME}" == "jenkins" ]; then
        DOCKER_IMAGE_NAME=$JENKINS_IMAGE_NAME
    fi

    # Get the last container image version
    LAST_VERSION=$(docker inspect --format='{{index .Config.Image}}' "$IMAGE_NAME" | awk -F ':' '{print $2}')

    # Define the Docker Hub API URL for the image
    API_URL="https://hub.docker.com/v2/repositories/$DOCKER_IMAGE_NAME/tags/?page_size=50&page=1"

    # Fetch the tags from Docker Hub API
    TAGS=$(curl -sSL "$API_URL" | jq -r '.results[].name')

    # Filter the tags to include only those containing "-alpine"
    ALPINE_TAGS=$(echo "$TAGS" | grep -E '^[0-9]+\.[0-9]+(\.[0-9]+)?-alpine(\-jdk21)?$')

    # Get the latest tag from the filtered list
    LATEST_VERSION=$(echo "$ALPINE_TAGS" | sort -V | tail -n1)

    # Print the latest tag
    echo "Latest tag for $DOCKER_IMAGE_NAME with -alpine: $LATEST_VERSION"

    # Compare the versions
    if [[ "$LAST_VERSION" == "$LATEST_VERSION" ]]; then
        echo "The container image is up to date."
    elif [ -n "${LAST_VERSION}" ]; then
        echo "The container image is outdated. Latest version: $LATEST_VERSION"
        docker rm -f "$IMAGE_NAME"
    fi

    # Check if the container exists
    if docker ps -a --format '{{.Names}}' | grep -q "^$IMAGE_NAME$"; then
        # Get the current state of the container
        STATE=$(docker inspect --format='{{.State.Status}}' "$IMAGE_NAME")

        # Check if the container is in the "starting" state
        if [ "$STATE" = "running" ]; then
            echo "Container $IMAGE_NAME is running."
        else
            echo "Container $IMAGE_NAME is not starting. Starting ..."
            # Start the existing container
            docker start "$IMAGE_NAME"
        fi
    else
        echo "Container '$IMAGE_NAME' does not exist."

        if [ -n "${IMAGE_NAME}" ] && [ "${IMAGE_NAME}" == "jenkins" ]; then
            IMAGE_NAME="custom-${IMAGE_NAME}"
            DOCKER_CONTAINER_NAME="jenkins"

            # Check if the Docker image exists
            if docker images --format "{{.Repository}}:{{.Tag}}" | grep -q "${IMAGE_NAME}:${LATEST_VERSION}"; then
                echo "Docker image '${IMAGE_NAME}:${LATEST_VERSION}' exists."
            else
                echo "Docker image '${IMAGE_NAME}:${LATEST_VERSION}' does not exist."
                
                docker build \
                    --no-cache \
                    -f build/ci/jenkins/Dockerfile \
                    -t ${IMAGE_NAME}:${LATEST_VERSION} .
            fi

            docker run --privileged \
                -d \
                -p $CONTAINER_HTTP_PORT:$HTTP_PORT \
                -p $CONTAINER_TCP_PORT:$TCP_PORT \
                --name ${DOCKER_CONTAINER_NAME} \
                --network $NETWORK_NAME \
                --restart=on-failure \
                -v jenkins_home:/var/jenkins_home \
                -v /var/run/docker.sock:/var/run/docker.sock \
                $IMAGE_NAME:$LATEST_VERSION

                # -v /var/run/docker.sock:/var/run/docker.sock \
                # -v ~/Projects/rust/build/ci/jenkins/resolv.conf:/etc/resolv.conf \

            echo -e "\nJenkins initial admin password : "
            docker exec ${DOCKER_CONTAINER_NAME} sh -c "cat /var/jenkins_home/secrets/initialAdminPassword"
            echo -e "\nGenerate SSH Public Key"
            docker exec ${DOCKER_CONTAINER_NAME} sh -c 'if [ ! -f "~/.ssh/id_ed25519" ] || [ ! -f "~/.ssh/id_ed25519.pub" ]; then ssh-keygen -t ed25519 -C "kristianto.aditya@gmail.com" -f ~/.ssh/id_ed25519 -N "" -q; fi'
            echo -e "\nSSH Public Key : "
            docker exec ${DOCKER_CONTAINER_NAME} sh -c "cat ~/.ssh/id_ed25519.pub"
            # docker exec ${DOCKER_CONTAINER_NAME} /bin/sh sudo usermod -aG docker jenkins

            # Prompt the user to press Enter to continue
            read -p "Press Enter to continue..."
        elif [ -n "${IMAGE_NAME}" ] && [ "${IMAGE_NAME}" == "kafka-server" ]; then
            docker run -d \
                --name ${IMAGE_NAME} \
                --hostname kafka-server \
                --network $NETWORK_NAME \
                -e KAFKA_CFG_NODE_ID=0 \
                -e KAFKA_CFG_PROCESS_ROLES=controller,broker \
                -e KAFKA_CFG_LISTENERS=PLAINTEXT://:9092,CONTROLLER://:9093 \
                -e KAFKA_CFG_LISTENER_SECURITY_PROTOCOL_MAP=CONTROLLER:PLAINTEXT,PLAINTEXT:PLAINTEXT \
                -e KAFKA_CFG_CONTROLLER_QUORUM_VOTERS=0@kafka-server:9093 \
                -e KAFKA_CFG_CONTROLLER_LISTENER_NAMES=CONTROLLER \
                bitnami/kafka:latest
        elif [ -n "${IMAGE_NAME}" ] && [ "${IMAGE_NAME}" == "kafka-ui" ]; then
            docker run -d \
                --name ${IMAGE_NAME} \
                -p 8181:8080 \
                -e DYNAMIC_CONFIG_ENABLED=true provectuslabs/kafka-ui
        elif [ -n "${IMAGE_NAME}" ] && [ "${IMAGE_NAME}" == "portainer" ]; then
        echo "disini"
            docker volume create portainer_data
            echo "docker run -d \
                -p $CONTAINER_HTTP_PORT:$HTTP_PORT \
                -p $CONTAINER_TCP_PORT:$TCP_PORT \
                --name ${IMAGE_NAME} \
                --restart=always \
                -v /var/run/docker.sock:/var/run/docker.sock \
                -v portainer_data:/data \
                $DOCKER_IMAGE_NAME:$LATEST_VERSION"
            docker run -d \
                -p $CONTAINER_HTTP_PORT:$HTTP_PORT \
                -p $CONTAINER_TCP_PORT:$TCP_PORT \
                --name ${IMAGE_NAME} \
                --restart=always \
                -v /var/run/docker.sock:/var/run/docker.sock \
                -v portainer_data:/data \
                $DOCKER_IMAGE_NAME:$LATEST_VERSION
        elif [ -n "${IMAGE_NAME}" ] && [ "${IMAGE_NAME}" == "postgres" ]; then
            # PostgreSQL connection details
            PG_USER="${11}"      # PostgreSQL username
            PG_PASSWORD="${12}" # PostgreSQL password

            docker run -d \
                --name $IMAGE_NAME \
                --network $NETWORK_NAME \
                --ip $CONTAINER_IP \
                --restart unless-stopped \
                -p $CONTAINER_HTTP_PORT:$HTTP_PORT \
                -p $CONTAINER_TCP_PORT:$TCP_PORT \
                -e POSTGRES_USER=$PG_USER \
                -e POSTGRES_PASSWORD="$PG_PASSWORD" \
                -e POSTGRES_DB=postgres \
                -d postgres:17.2-alpine3.21 \
                -c shared_buffers=256MB \
                -c max_connections=200
        elif [ -n "${IMAGE_NAME}" ] && [ "${IMAGE_NAME}" == "pgadmin" ]; then
            # PostgreSQL connection details
            PGADMIN_EMAIL="${11}"     # PostgreSQL username
            PGADMIN_PASSWORD="${12}" # PostgreSQL password

            docker run -d \
                --name $IMAGE_NAME \
                --network $NETWORK_NAME \
                --ip $CONTAINER_IP \
                --restart unless-stopped \
                -p $CONTAINER_HTTP_PORT:$HTTP_PORT \
                -p $CONTAINER_TCP_PORT:$TCP_PORT \
                -e PGADMIN_DEFAULT_EMAIL=$PGADMIN_EMAIL \
                -e PGADMIN_DEFAULT_PASSWORD=$PGADMIN_PASSWORD \
                -d dpage/pgadmin4:9.1.0
        fi
    fi
fi

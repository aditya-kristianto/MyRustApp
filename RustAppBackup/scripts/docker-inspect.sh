#!/bin/bash

# Get all running container IDs
CONTAINER_IDS=$(docker ps -q)

# Iterate over each container ID
for container_id in $CONTAINER_IDS; do
    # Get the container name
    CONTAINER_NAME=$(docker inspect --format='{{.Name}}' "$container_id")

    # Get the IP address of the container
    IP_ADDRESS=$(docker inspect --format='{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$container_id")

    # Print container name and IP address
    echo "Container Name: ${CONTAINER_NAME:1}, IP Address: $IP_ADDRESS"
done

# Get list of all Docker networks
NETWORKS=$(docker network ls --format '{{.Name}}')

# Loop through each network
for network in $NETWORKS; do
    # Get subnet and gateway information
    NETWORK_INFO=$(docker network inspect "$network")
    
    # Parse subnet and gateway information from the output
    SUBNET=$(echo "$NETWORK_INFO" | jq -r '.[0].IPAM.Config[0].Subnet')
    GATEWAY=$(echo "$NETWORK_INFO" | jq -r '.[0].IPAM.Config[0].Gateway')
    
    # Print network name, subnet, and gateway information
    echo "Network: $network"
    echo "  Subnet: $SUBNET"
    echo "  Gateway: $GATEWAY"
done
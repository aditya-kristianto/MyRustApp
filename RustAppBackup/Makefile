DOCKER_IMAGE_VERSION = 1.0.0
DOCKER_USERNAME = kristianto.aditya@gmail.com
DOCKER_NETWORK = rust-network
DOCKER_SUBNET = 172.18.0.0/24
DOCKER_GATEWAY = 172.18.0.1
# version number
TARGET = release
VERBOSE_OUTPUT = true
# debug | release
BUILD_MULTIPLATFORM = true
build_dir_alpine_name=rust-alpine
build_dir_app_name=rust-app
build_dir_asset_name=rust-asset
build_dir_ubuntu_name=docker-ubuntu
app_name = rust
app_name_alpine = rust-alpine
app_name_asset_service = asset
app_name_auth_service = auth
app_name_consumer_service = consumer
app_name_oauth_service = oauth
app_name_producer_service = producer
app_name_scheduler_service = scheduler
app_name_stock_service = stock
app_name_user_service = user
app_name_uuid_service = uuid
app_name_web_service = web
bin_1 = web
bin_asset_service = asset
bin_auth_service = auth
bin_consumer_service = consumer
bin_oauth_service = oauth
bin_producer_service = producer
bin_stock_service = stock
bin_user_service = user
bin_uuid_service = uuid
bin_web_service = web
container_ip_asset_service = 172.18.0.4
container_ip_auth_service = 172.18.0.5
container_ip_oauth_service = 172.18.0.6
container_ip_stock_service = 172.18.0.7
container_ip_uuid_service = 172.18.0.8
container_ip_web_service = 172.18.0.9
container_ip_postgres = 172.18.0.2
container_ip_pgadmin = 172.18.0.3
container_name = rust-app
container_name_alpine = rust-alpine
container_name_asset_service = rust-asset
container_name_auth_service = rust-auth
container_name_consumer_service = rust-consumer
container_name_oauth_service = rust-oauth
container_name_producer_service = rust-producer
container_name_scheduler_service = rust-scheduler
container_name_stock_service = rust-stock
container_name_user_service = rust-user
container_name_uuid_service = rust-uuid
container_name_web_service = rust-web
image_name = rust-app
image_name_alpine = rust-alpine
image_name_asset_service = rust-asset
image_name_auth_service = rust-auth
image_name_consumer_service = rust-consumer
image_name_oauth_service = rust-oauth
image_name_producer_service = rust-producer
image_name_scheduler_service = rust-scheduler
image_name_stock_service = rust-stock
image_name_user_service = rust-user
image_name_ubuntu = docker-ubuntu
image_name_uuid_service = rust-uuid
image_name_web_service = rust-web
AWS_ACCOUNT_ID = 473154593366
AWS_REGION = ap-southeast-1
container_port = 8080

# Project Management
# =======================================================
APP_NAME_EMPLOYEE_SERVICE=employee
APP_NAME_MAIL_SERVICE=mail
APP_NAME_PROJECT_SERVICE=project

BIN_EMPLOYEE_SERVICE=employee
BIN_MAIL_SERVICE=mail
BIN_PROJECT_SERVICE=project

CONTAINER_IP_EMPLOYEE_SERVICE=172.18.0.11
CONTAINER_IP_MAIL_SERVICE=172.18.0.12
CONTAINER_IP_PROJECT_SERVICE=172.18.0.13

CONTAINER_NAME_EMPLOYEE_SERVICE=rust-employee
CONTAINER_NAME_MAIL_SERVICE=rust-mail
CONTAINER_NAME_PROJECT_SERVICE=rust-project

IMAGE_NAME_EMPLOYEE_SERVICE=rust-employee
IMAGE_NAME_MAIL_SERVICE=rust-mail
IMAGE_NAME_PROJECT_SERVICE=rust-project
# =======================================================
# End - Project Management

# Jenkins
container_http_port_jenkins_service = 80
container_https_port_jenkins_service = 50000
host_http_port_jenkins_service = 8080
host_https_port_jenkins_service = 50000

# Portainer
container_http_port_portainer_service = 8003
container_https_port_portainer_service = 9443
host_http_port_portainer_service = 8004
host_https_port_portainer_service = 9443

container_http_port_asset_service = 8081
CONTAINER_HTTP_PORT_AUTH_SERVICE = 8082
container_http_port_oauth_service = 8083
container_http_port_stock_service = 8084
container_http_port_uuid_service = 8085
container_http_port_web_service = 8080
container_TCP_PORT_asset_service = 9091
CONTAINER_TCP_PORT_AUTH_SERVICE = 9092
container_TCP_PORT_oauth_service = 9093
container_TCP_PORT_stock_service = 9094
container_TCP_PORT_uuid_service = 9095
container_TCP_PORT_web_service = 9090
# Project Management
# =======================================================
CONTAINER_HTTP_PORT_EMPLOYEE_SERVICE=8086
CONTAINER_HTTP_PORT_MAIL_SERVICE=8087
CONTAINER_HTTP_PORT_PROJECT_SERVICE=8088

CONTAINER_TCP_PORT_EMPLOYEE_SERVICE=9096
CONTAINER_TCP_PORT_MAIL_SERVICE=9097
CONTAINER_TCP_PORT_PROJECT_SERVICE=9098
# =======================================================
# End - Project Management

HOST_PORT = 80
TCP_PORT = 8080
DOCKER_IMAGE_NAME = $(app_name):$(DOCKER_IMAGE_VERSION)-$(TARGET)
ASSET_NAME=metronic
ASSET_VERSION=v8.2.9
TEMPLATE_TYPE = demo1
IS_ECR = true

PG_USER=aditya.kristianto
PG_PASSWORD=my secret password
PG_CONTAINER_PORT="5432"
PG_HOST_PORT="5432"
PG_CONTAINER_TCP_PORT="9432"
PG_TCP_PORT="9432"
PGADMIN_CONTAINER_PORT="5431"
PGADMIN_HOST_PORT="80"
PGADMIN_CONTAINER_TCP_PORT="9431"
PGADMIN_TCP_PORT="9431"
PGADMIN_EMAIL="kristianto.aditya@gmail.com"
PGADMIN_PASSWORD="mysecretpassword"
DATABASE_NAME="rust_app"   			# Name of the database to create
DATABASE_AUTH_NAME="rust_auth"
DATABASE_OAUTH_NAME="rust_oauth"
DATABASE_STOCK_NAME="rust_stock"
DATABASE_UUID_NAME="rust_uuid"
DATABASE_MIGRATION_AUTH_DIR="db/migrations/auth"
DATABASE_MIGRATION_OAUTH_DIR="db/migrations/oauth"
DATABASE_MIGRATION_STOCK_DIR="db/migrations/stock"
DATABASE_MIGRATION_UUID_DIR="db/migrations/uuid"

S3_BUCKET=s3://rust-assets-bucket/

# Define the number of versions to retain
VERSIONS_TO_RETAIN = 5

# Define a variable outside of any TARGET
VAR_OUTSIDE := Hello from outside

# Define the ANSI escape code for red color
RED='\033[0;31m'
# Reset color
NC='\033[0m'

export AWS_ACCESS_KEY_ID=AKIAW4KRWEJLKU4KF76V
export AWS_SECRET_ACCESS_KEY=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

define cargo-check
	cargo check
endef

define cargo-update
	cargo update
endef

define cargo-upgrade
	cargo upgrade
endef

define clear
	@if [ -t 1 ]; then clear; fi
endef

define docker-build
	$(docker-login)
	$(docker-build-rust-asset)
	$(docker-build-rust-auth)
	$(docker-build-rust-consumer)
	$(docker-build-rust-oauth)
	$(docker-build-rust-producer)
	$(docker-build-rust-stock)
	$(docker-build-rust-user)
	$(docker-build-rust-uuid)
	$(docker-build-rust-web)
endef

define docker-build-rust-alpine
	$(clear)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} "" ${app_name_alpine} $(image_name_alpine) $(container_name_alpine) "" "" $(TARGET) $(TEMPLATE_TYPE) false true $(VERBOSE_OUTPUT)
endef

define docker-build-rust-asset
	$(clear)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} ${bin_asset_service} ${app_name_asset_service} $(image_name_asset_service) $(container_name_asset_service) ${HOST_PORT} ${TCP_PORT} $(TARGET) $(TEMPLATE_TYPE) true false $(VERBOSE_OUTPUT) $(ASSET_NAME) $(ASSET_VERSION)
endef

define docker-build-rust-auth
	$(clear)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} ${bin_auth_service} ${app_name_auth_service} $(image_name_auth_service) $(container_name_auth_service) ${HOST_PORT} ${TCP_PORT} $(TARGET) $(TEMPLATE_TYPE) false false $(VERBOSE_OUTPUT) $(ASSET_NAME) $(ASSET_VERSION)
endef

define docker-build-rust-consumer
	$(clear)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} ${bin_consumer_service} ${app_name_consumer_service} $(image_name_consumer_service) $(container_name_consumer_service) ${HOST_PORT} ${TCP_PORT} $(TARGET) $(TEMPLATE_TYPE) false false $(VERBOSE_OUTPUT) $(ASSET_NAME) $(ASSET_VERSION)
endef

define docker-build-rust-migration
	$(clear)
	docker build --no-cache -f build/ci/rust-migration/Dockerfile -t rust-migration:1.0.6 .
	docker tag rust-migration:1.0.6 473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/rust-migration:1.0.6
endef

define docker-build-rust-oauth
	$(clear)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} ${bin_oauth_service} ${app_name_oauth_service} $(image_name_oauth_service) $(container_name_oauth_service) ${HOST_PORT} ${TCP_PORT} $(TARGET) $(TEMPLATE_TYPE) false false $(VERBOSE_OUTPUT) $(ASSET_NAME) $(ASSET_VERSION)
endef

define docker-build-rust-producer
	$(clear)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} ${bin_producer_service} ${app_name_producer_service} $(image_name_producer_service) $(container_name_producer_service) ${HOST_PORT} ${TCP_PORT} $(TARGET) $(TEMPLATE_TYPE) false false $(VERBOSE_OUTPUT) $(ASSET_NAME) $(ASSET_VERSION)
endef

define docker-build-rust-scheduler
	$(clear)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} ${bin_scheduler_service} ${app_name_scheduler_service} $(image_name_scheduler_service) $(container_name_scheduler_service) ${HOST_PORT} ${TCP_PORT} $(TARGET) $(TEMPLATE_TYPE) false false $(VERBOSE_OUTPUT) $(ASSET_NAME) $(ASSET_VERSION)
endef

define docker-build-rust-stock
	$(clear)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} ${bin_stock_service} ${app_name_stock_service} $(image_name_stock_service) $(container_name_stock_service) ${HOST_PORT} ${TCP_PORT} $(TARGET) $(TEMPLATE_TYPE) false false $(VERBOSE_OUTPUT) $(ASSET_NAME) $(ASSET_VERSION)
endef

define docker-build-rust-user
	$(clear)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} ${bin_user_service} ${app_name_user_service} $(image_name_user_service) $(container_name_user_service) ${HOST_PORT} ${TCP_PORT} $(TARGET) $(TEMPLATE_TYPE) false false $(VERBOSE_OUTPUT) $(ASSET_NAME) $(ASSET_VERSION)
endef

define docker-build-rust-uuid
	$(clear)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} ${bin_uuid_service} ${app_name_uuid_service} $(image_name_uuid_service) $(container_name_uuid_service) ${HOST_PORT} ${TCP_PORT} $(TARGET) $(TEMPLATE_TYPE) false false $(VERBOSE_OUTPUT) $(ASSET_NAME) $(ASSET_VERSION)
endef

define docker-build-rust-web
	$(clear)
	$(minify)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} ${bin_web_service} ${app_name_web_service} $(image_name_web_service) $(container_name_web_service) ${HOST_PORT} ${TCP_PORT} $(TARGET) $(TEMPLATE_TYPE) false false $(VERBOSE_OUTPUT) $(ASSET_NAME) $(ASSET_VERSION)
endef

# Project Management
# =======================================================
define docker-build-rust-project-management
	$(docker-build-rust-employee)
	$(docker-build-rust-mail)
	$(docker-build-rust-project)
endef

define docker-build-rust-employee
	$(clear)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} ${BIN_EMPLOYEE_SERVICE} ${APP_NAME_EMPLOYEE_SERVICE} $(IMAGE_NAME_EMPLOYEE_SERVICE) $(CONTAINER_NAME_EMPLOYEE_SERVICE) ${HOST_PORT} ${TCP_PORT} $(TARGET) $(TEMPLATE_TYPE) false false $(VERBOSE_OUTPUT) $(ASSET_NAME) $(ASSET_VERSION)
endef

define docker-build-rust-mail
	$(clear)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} ${BIN_MAIL_SERVICE} ${APP_NAME_MAIL_SERVICE} $(IMAGE_NAME_MAIL_SERVICE) $(CONTAINER_NAME_MAIL_SERVICE) ${HOST_PORT} ${TCP_PORT} $(TARGET) $(TEMPLATE_TYPE) false false $(VERBOSE_OUTPUT) $(ASSET_NAME) $(ASSET_VERSION)
endef

define docker-build-rust-project
	$(clear)
	./scripts/docker-build-image.sh ${AWS_ACCOUNT_ID} ${AWS_REGION} ${BUILD_MULTIPLATFORM} ${BIN_PROJECT_SERVICE} ${APP_NAME_PROJECT_SERVICE} $(IMAGE_NAME_PROJECT_SERVICE) $(CONTAINER_NAME_PROJECT_SERVICE) ${HOST_PORT} ${TCP_PORT} $(TARGET) $(TEMPLATE_TYPE) false false $(VERBOSE_OUTPUT) $(ASSET_NAME) $(ASSET_VERSION)
endef
# =======================================================
# End - Project Management

define docker-login
	aws ecr get-login-password --region ap-southeast-1 | docker login --username AWS --password-stdin 473154593366.dkr.ecr.ap-southeast-1.amazonaws.com
endef

define docker-push
	$(docker-push-rust-alpine)
	$(docker-push-rust-asset)
	$(docker-push-rust-auth)
	$(docker-push-rust-oauth)
	$(docker-push-rust-stock)
	$(docker-push-rust-uuid)
	$(docker-push-rust-web)
endef

define docker-push-rust-alpine
	./scripts/docker-push.sh $(AWS_ACCOUNT_ID) ${AWS_REGION} $(image_name_alpine) $(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define docker-push-rust-asset
	./scripts/docker-push.sh $(AWS_ACCOUNT_ID) ${AWS_REGION} $(image_name_asset_service) $(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define docker-push-rust-auth
	./scripts/docker-push.sh $(AWS_ACCOUNT_ID) ${AWS_REGION} $(image_name_auth_service) $(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define docker-push-rust-migration
	docker push ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/rust-migration:1.0.6
endef

define docker-push-rust-oauth
	./scripts/docker-push.sh $(AWS_ACCOUNT_ID) ${AWS_REGION} $(image_name_oauth_service) $(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define docker-push-rust-stock
	./scripts/docker-push.sh $(AWS_ACCOUNT_ID) ${AWS_REGION} $(image_name_stock_service) $(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define docker-push-rust-uuid
	./scripts/docker-push.sh $(AWS_ACCOUNT_ID) ${AWS_REGION} $(image_name_uuid_service) $(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define docker-push-rust-web
	./scripts/docker-push.sh $(AWS_ACCOUNT_ID) ${AWS_REGION} $(image_name_web_service) $(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define docker-rm-rust-stock
	docker stop rust_stock_app && docker rm rust_stock_app
endef

define docker-rm-rust-web
	docker stop rust_web_app && docker rm rust_web_app
endef

define docker-container-stop
	$(docker-container-stop-asset)
	$(docker-container-stop-auth)
	$(docker-container-stop-oauth)
	$(docker-container-stop-stock)
	$(docker-container-stop-uuid)
	$(docker-container-stop-web)
endef

define docker-container-stop-asset
	if docker ps -q --filter "name=$(container_name_asset_service):$(DOCKER_IMAGE_VERSION)-$(TARGET)" | grep -q . ; then \
		docker stop $(container_name_asset_service):$(DOCKER_IMAGE_VERSION)-$(TARGET); \
		echo "Container $(container_name_asset_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) stopped."; \
	else \
		echo "Container $(container_name_asset_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) is not running."; \
	fi
endef

define docker-container-stop-auth
	if docker ps -q --filter "name=$(container_name_auth_service):$(DOCKER_IMAGE_VERSION)-$(TARGET)" | grep -q . ; then \
		docker stop $(container_name_auth_service):$(DOCKER_IMAGE_VERSION)-$(TARGET); \
		echo "Container $(container_name_auth_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) stopped."; \
	else \
		echo "Container $(container_name_auth_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) is not running."; \
	fi
endef

define docker-container-stop-oauth
	if docker ps -q --filter "name=$(container_name_oauth_service):$(DOCKER_IMAGE_VERSION)-$(TARGET)" | grep -q . ; then \
		docker stop $(container_name_oauth_service):$(DOCKER_IMAGE_VERSION)-$(TARGET); \
		echo "Container $(container_name_oauth_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) stopped."; \
	else \
		echo "Container $(container_name_oauth_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) is not running."; \
	fi
endef

define docker-container-stop-stock
	if docker ps -q --filter "name=$(container_name_asset_service):$(DOCKER_IMAGE_VERSION)-$(TARGET)" | grep -q . ; then \
		docker stop $(container_name_asset_service):$(DOCKER_IMAGE_VERSION)-$(TARGET); \
		echo "Container $(container_name_asset_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) stopped."; \
	else \
		echo "Container $(container_name_asset_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) is not running."; \
	fi
endef

define docker-container-stop-uuid
	if docker ps -q --filter "name=$(container_name_uuid_service):$(DOCKER_IMAGE_VERSION)-$(TARGET)" | grep -q . ; then \
		docker stop $(container_name_uuid_service):$(DOCKER_IMAGE_VERSION)-$(TARGET); \
		echo "Container $(container_name_uuid_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) stopped."; \
	else \
		echo "Container $(container_name_uuid_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) is not running."; \
	fi
endef

define docker-container-stop-web
	if docker ps -q --filter "name=$(container_name_web_service):$(DOCKER_IMAGE_VERSION)-$(TARGET)" | grep -q . ; then \
		docker stop $(container_name_web_service):$(DOCKER_IMAGE_VERSION)-$(TARGET); \
		echo "Container $(container_name_web_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) stopped."; \
	else \
		echo "Container $(container_name_web_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) is not running."; \
	fi
endef

define docker-remove
	$(docker-remove-rust-alpine)
	$(docker-remove-asset)
	$(docker-remove-auth)
	$(docker-remove-oauth)
	$(docker-remove-stock)
	$(docker-remove-uuid)
	$(docker-remove-web)
endef

define docker-remove-asset
	scripts/docker-remove-image.sh $(app_name)-$(bin_asset_service) ${VERSIONS_TO_RETAIN}
endef

define docker-remove-auth
	scripts/docker-remove-image.sh $(app_name)-$(bin_auth_service) ${VERSIONS_TO_RETAIN}
endef

define docker-remove-oauth
	scripts/docker-remove-image.sh $(app_name)-$(bin_oauth_service) ${VERSIONS_TO_RETAIN}
endef

define docker-remove-stock
	scripts/docker-remove-image.sh $(app_name)-$(bin_stock_service) ${VERSIONS_TO_RETAIN}
endef

define docker-remove-uuid
	scripts/docker-remove-image.sh $(app_name)-$(bin_uuid_service) ${VERSIONS_TO_RETAIN}
endef

define docker-remove-web
	scripts/docker-remove-image.sh $(app_name)-$(bin_web_service) ${VERSIONS_TO_RETAIN}
endef

define docker-remove-rust-alpine
	scripts/docker-remove-image.sh ${app_name_alpine} ${VERSIONS_TO_RETAIN}
endef

define docker-run
	$(docker-run-jenkins)
	$(docker-run-kafka)
	$(docker-run-kafka-ui)
	$(docker-run-portainer)
	$(docker-run-elasticsearch)
	$(docker-run-postgres)
	$(docker-run-pgadmin)
	$(docker-run-rabbitmq)
	$(docker-run-redis)
	$(docker-run-redisinsight)
	$(docker-run-tyk)
	$(docker-run-rust-asset)
	$(docker-run-rust-auth)
	$(docker-run-rust-consumer)
	$(docker-run-rust-oauth)
	$(docker-run-rust-producer)
	$(docker-run-rust-stock)
	$(docker-run-rust-uuid)
	$(docker-run-rust-web)
endef

define docker-run-elasticsearch
	# docker network create app-tier --driver bridge
	docker run -d --name elasticsearch-server \
		-p 9200:9200 \
		-p 9300:9300 \
		--network app-tier \
		bitnami/elasticsearch:latest
endef

define docker-run-jenkins
	./scripts/docker-run.sh jenkins ${container_http_port_jenkins_service} ${container_https_port_jenkins_service} ${host_http_port_jenkins_service} ${host_https_port_jenkins_service} true ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${container_ip_postgres} ${PG_USER} ${PG_PASSWORD} ${DATABASE_NAME}
endef

define docker-run-kafka
	./scripts/docker-run.sh kafka-server ${container_http_port_jenkins_service} ${container_https_port_jenkins_service} ${host_http_port_jenkins_service} ${host_https_port_jenkins_service} true ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${container_ip_postgres} ${PG_USER} ${PG_PASSWORD} ${DATABASE_NAME}
	# docker-compose -f build/ci/kafka/development/docker-compose.yml \
	# 	-p kafka up -d
	# docker network create app-tier --driver bridge
	
	# docker run -it --rm \
	# 	--network app-tier \
	# 	bitnami/kafka:latest kafka-topics.sh --list  --bootstrap-server kafka-server:9092
endef

define docker-run-kafka-exporter
	docker network create kafka-exporter-network --driver bridge
	docker run -d \
		--name kafka-exporter-node1 \
		--network kafka-exporter-network \
		bitnami/kafka-exporter:latest
endef

define docker-run-kafka-ui
	./scripts/docker-run.sh kafka-ui ${container_http_port_kafka_ui_service} ${container_https_port_kafka_ui_service} ${host_http_port_kafka_ui_service} ${host_https_port_kafka_ui_service} true ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${container_ip_kafka_ui} ${PG_USER} ${PG_PASSWORD} ${DATABASE_NAME}
endef

define docker-run-kong
	docker pull kong/kong-gateway:latest
	docker run -d \
		--name kong \
		-e "KONG_PG_HOST=172.17.0.2" \
		-e "KONG_PG_PORT=5432" \
		-e "KONG_PG_USER=aditya.kristianto" \
		-e "KONG_PG_PASSWORD=my secret password" \
		-p 8000:8000 \
		-p 8001:8001 \
		-p 8002:8002 \
		kong/kong-gateway:latest
		# /bin/sh -c "kong migrations finish"
endef

define docker-run-mongodb-community-server
	docker run -d \
		-p 27017:27017 \
		--name mongodb \
		-e MONGO_INITDB_ROOT_USERNAME=root \
		-e MONGO_INITDB_ROOT_PASSWORD=password \
		mongodb/mongodb-community-server
endef

define docker-run-portainer
	./scripts/docker-run.sh portainer ${container_http_port_portainer_service} ${host_http_port_portainer_service} ${container_https_port_portainer_service} ${host_https_port_portainer_service} true ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${container_ip_postgres} ${PG_USER} ${PG_PASSWORD} ${DATABASE_NAME}
endef

define docker-run-postgres
	./scripts/docker-run.sh postgres ${PG_CONTAINER_PORT} ${PG_CONTAINER_TCP_PORT} ${PG_HOST_PORT} ${PG_TCP_PORT} true ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${container_ip_postgres} ${PG_USER} ${PG_PASSWORD} ${DATABASE_NAME}
endef

define docker-run-pgadmin
	./scripts/docker-run.sh pgadmin ${PGADMIN_CONTAINER_PORT} ${PGADMIN_CONTAINER_TCP_PORT} ${PGADMIN_HOST_PORT} ${PGADMIN_TCP_PORT} true ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${container_ip_pgadmin} ${PGADMIN_EMAIL} ${PGADMIN_PASSWORD}
endef

define docker-run-rabbitmq
	docker run -d --hostname my-rabbit \
		--name some-rabbit \
		rabbitmq:3.9.29-alpine
endef

define docker-run-redis
	docker run -d \
		--name redis \
		-p 6379:6379 \
		redis:7.2.0-alpine3.18 \
		redis-server \
		--save 60 1 \
		--loglevel warning
endef
	
define docker-run-redisinsight
	docker run --name redisinsight \
		-d -e RIPROXYENABLE=true \
		-v redisinsight:/db \
		-p 8001:8001 \
		redislabs/redisinsight:1.14.0
endef

define docker-run-rust-asset
	./scripts/docker-run.sh $(app_name)-$(bin_asset_service) ${container_http_port_asset_service} ${container_TCP_PORT_asset_service} ${HOST_PORT} ${TCP_PORT} false ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${container_ip_asset_service}
endef

define docker-run-rust-auth
	./scripts/docker-run.sh $(app_name)-$(bin_auth_service) ${CONTAINER_HTTP_PORT_AUTH_SERVICE} ${CONTAINER_TCP_PORT_AUTH_SERVICE} ${HOST_PORT} ${TCP_PORT} false ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${container_ip_auth_service} ${PG_USER} ${PG_PASSWORD} ${DATABASE_AUTH_NAME} ${DATABASE_MIGRATION_AUTH_DIR}
endef

define docker-run-rust-oauth
	./scripts/docker-run.sh $(app_name)-$(bin_oauth_service) ${container_http_port_oauth_service} ${container_TCP_PORT_oauth_service} ${HOST_PORT} ${TCP_PORT} false ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${container_ip_oauth_service} ${PG_USER} ${PG_PASSWORD} ${DATABASE_OAUTH_NAME} ${DATABASE_MIGRATION_OAUTH_DIR}
endef

define docker-run-rust-stock
	./scripts/docker-run.sh $(app_name)-$(bin_stock_service) ${container_http_port_stock_service} ${container_TCP_PORT_stock_service} ${HOST_PORT} ${TCP_PORT} false ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${container_ip_stock_service} ${PG_USER} ${PG_PASSWORD} ${DATABASE_STOCK_NAME} ${DATABASE_MIGRATION_STOCK_DIR}
endef
	
define docker-run-rust-uuid
	./scripts/docker-run.sh $(app_name)-$(bin_uuid_service) ${container_http_port_uuid_service} ${container_TCP_PORT_uuid_service} ${HOST_PORT} ${TCP_PORT} false ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${container_ip_uuid_service} ${PG_USER} ${PG_PASSWORD} ${DATABASE_UUID_NAME} ${DATABASE_MIGRATION_UUID_DIR}
endef

define docker-run-rust-web
	./scripts/docker-run.sh $(app_name)-$(bin_web_service) ${container_http_port_web_service} ${container_TCP_PORT_web_service} ${HOST_PORT} ${TCP_PORT} false ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${container_ip_web_service}
endef

# Project Management
# =======================================================
define docker-run-rust-employee
	./scripts/docker-run.sh $(app_name)-$(BIN_EMPLOYEE_SERVICE) ${CONTAINER_HTTP_PORT_EMPLOYEE_SERVICE} ${CONTAINER_TCP_PORT_EMPLOYEE_SERVICE} ${HOST_PORT} ${TCP_PORT} false ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${CONTAINER_IP_EMPLOYEE_SERVICE}
endef

define docker-run-rust-mail
	./scripts/docker-run.sh $(app_name)-$(BIN_MAIL_SERVICE) ${CONTAINER_HTTP_PORT_MAIL_SERVICE} ${CONTAINER_TCP_PORT_MAIL_SERVICE} ${HOST_PORT} ${TCP_PORT} false ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${CONTAINER_IP_MAIL_SERVICE}
endef

define docker-run-rust-project
	./scripts/docker-run.sh $(app_name)-$(BIN_PROJECT_SERVICE) ${CONTAINER_HTTP_PORT_PROJECT_SERVICE} ${CONTAINER_TCP_PORT_PROJECT_SERVICE} ${HOST_PORT} ${TCP_PORT} false ${DOCKER_NETWORK} ${DOCKER_SUBNET} ${DOCKER_GATEWAY} ${CONTAINER_IP_PROJECT_SERVICE}
endef

define docker-run-rust-project-management
	$(docker-run-rust-employee)
	$(docker-run-rust-mail)
	$(docker-run-rust-project)
endef
# =======================================================
# End - Project Management

define docker-run-tyk
	# git clone https://github.com/TykTechnologies/tyk-gateway-docker
	# cd tyk-gateway-docker
	# docker-compose up -d
	# curl localhost:8080/hello
	#wget https://raw.githubusercontent.com/TykTechnologies/tyk-gateway-docker/master/tyk.standalone.conf
	
	# wget https://raw.githubusercontent.com/TykTechnologies/tyk-gateway-docker/master/tyk.standalone.conf
	# docker network create tyk
	# docker run -itd --rm -d \
  	# 	--name tyk_gateway \
	# 	--network tyk \
  	# 	-p 8080:8080 \
  	# 	-v $(pwd)/tyk.standalone.conf:/opt/tyk-gateway/tyk.conf \
  	# 	docker.tyk.io/tyk-gateway/tyk-gateway:latest
	# -v $(pwd)/apps:/opt/tyk-gateway/apps
endef

define docker-start
	$(docker-build)
	$(docker-remove)
	$(docker-run)
endef

define docker-buildx-build
	docker buildx build --build-arg TARGET=$(TARGET) --build-arg TEMPLATE_TYPE=$(TEMPLATE_TYPE) --no-cache -f build/ci/$(app_name)/Dockerfile -t $(app_name):$(DOCKER_IMAGE_VERSION)-$(TARGET) .
endef

define cloudbuild-ecr-remove-asset-images
endef

define cloudbuild-docker-build
	docker build \
		--build-arg bin=$(bin_auth_service) \
		--build-arg TARGET=$(TARGET) \
		--build-arg app_name=$(app_name) \
		--build-arg expose_port=${HOST_PORT} \
		--build-arg TEMPLATE_TYPE=$(TEMPLATE_TYPE) \
		--no-cache \
		-f build/ci/$(app_name)/Dockerfile \
		-t $(app_name):$(DOCKER_IMAGE_VERSION)-$(TARGET) .
	docker tag $(app_name):$(DOCKER_IMAGE_VERSION)-$(TARGET) $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com/$(app_name):$(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define cloudbuild-docker-asset-build
	docker build \
		--build-arg bin=$(bin_asset_service) \
		--build-arg TARGET=$(TARGET) \
		--build-arg app_name=$(app_name_asset_service) \
		--build-arg expose_port=${HOST_PORT} \
		--build-arg TEMPLATE_TYPE=$(TEMPLATE_TYPE) \
		--no-cache \
		-f build/ci/$(app_name)/Dockerfile \
		-t $(app_name_asset_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) .
	docker tag $(app_name_asset_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com/$(app_name_asset_service):$(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define cloudbuild-docker-oauth-build
	docker build \
		--build-arg bin=$(bin_oauth_service) \
		--build-arg TARGET=$(TARGET) \
		--build-arg app_name=$(app_name_oauth_service) \
		--build-arg expose_port=${HOST_PORT} \
		--build-arg TEMPLATE_TYPE=$(TEMPLATE_TYPE) \
		--no-cache \
		-f build/ci/$(app_name)/Dockerfile \
		-t $(app_name_oauth_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) .
	docker tag $(app_name_oauth_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com/$(app_name_oauth_service):$(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define cloudbuild-docker-stock-build
	docker build \
		--build-arg bin=$(bin_stock_service) \
		--build-arg TARGET=$(TARGET) \
		--build-arg app_name=$(app_name_stock_service) \
		--build-arg expose_port=${HOST_PORT} \
		--build-arg TEMPLATE_TYPE=$(TEMPLATE_TYPE) \
		--no-cache \
		-f build/ci/$(app_name)/Dockerfile \
		-t $(app_name_stock_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) .
	docker tag $(app_name_stock_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com/$(app_name_stock_service):$(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define cloudbuild-docker-uuid-build
	docker build \
		--build-arg bin=$(bin_uuid_service) \
		--build-arg TARGET=$(TARGET) \
		--build-arg app_name=$(app_name_uuid_service) \
		--build-arg expose_port=${HOST_PORT} \
		--build-arg TEMPLATE_TYPE=$(TEMPLATE_TYPE) \
		--no-cache \
		-f build/ci/$(app_name)/Dockerfile \
		-t $(app_name_uuid_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) .
	docker tag $(app_name_uuid_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com/$(app_name_uuid_service):$(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define cloudbuild-docker-web-build
	docker build \
		--build-arg bin=$(bin_web_service) \
		--build-arg TARGET=$(TARGET) \
		--build-arg app_name=$(app_name_web_service) \
		--build-arg expose_port=${HOST_PORT} \
		--build-arg TEMPLATE_TYPE=$(TEMPLATE_TYPE) \
		--no-cache \
		-f build/ci/$(app_name)/Dockerfile \
		-t $(app_name_web_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) .
	docker tag $(app_name_web_service):$(DOCKER_IMAGE_VERSION)-$(TARGET) $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com/$(app_name_web_service):$(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define docker-buildx-build-using-cloudbuild
	docker buildx build --build-arg TARGET=$(TARGET) --build-arg app_name=$(app_name) --build-arg TEMPLATE_TYPE=$(TEMPLATE_TYPE) --no-cache -f build/codebuild/$(app_name)/Dockerfile -t $(app_name):$(DOCKER_IMAGE_VERSION)-$(TARGET) .
	docker tag $(app_name):$(DOCKER_IMAGE_VERSION)-$(TARGET) $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com/$(app_name):$(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define cloudbuild-docker-push
	docker push $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com/$(app_name):$(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define cloudbuild-docker-asset-push
	./scripts/docker-push.sh $(AWS_ACCOUNT_ID) ${AWS_REGION} $(image_name_asset_service) $(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define cloudbuild-docker-auth-push
	./scripts/docker-push.sh $(AWS_ACCOUNT_ID) ${AWS_REGION} $(image_name_auth_service) $(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define cloudbuild-docker-oauth-push
	./scripts/docker-push.sh $(AWS_ACCOUNT_ID) ${AWS_REGION} $(image_name_oauth_service) $(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define cloudbuild-docker-stock-push
	./scripts/docker-push.sh $(AWS_ACCOUNT_ID) ${AWS_REGION} $(image_name_stock_service) $(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define cloudbuild-docker-uuid-push
	./scripts/docker-push.sh $(AWS_ACCOUNT_ID) ${AWS_REGION} $(image_name_uuid_service) $(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define cloudbuild-docker-web-push
	./scripts/docker-push.sh $(AWS_ACCOUNT_ID) ${AWS_REGION} $(image_name_web_service) $(DOCKER_IMAGE_VERSION)-$(TARGET)
endef

define generate-templates
	rm -rf templates
	mkdir -p templates
	cp -R static/${TEMPLATE_TYPE}/dist/* templates/
endef

define kubectl-apply
	cp deployments/$(app_name).yaml deployments/$(app_name)-temp.yaml
	sed -i '' 's/SERVICE_NAME/$(app_name)/g' deployments/$(app_name)-temp.yaml
	sed -i '' 's/DOCKER_IMAGE/$(DOCKER_IMAGE_NAME)/g' deployments/$(app_name)-temp.yaml
	kubectl apply -f deployments/$(app_name)-temp.yaml
	rm -rf deployments/$(app_name)-temp.yaml
endef

define minify
	./scripts/minify.sh static/${ASSET_NAME}/${ASSET_VERSION} "UGLIFYJS" ${S3_BUCKET} ${TEMPLATE_TYPE}
endef

define rust-update
	rustup update stable
endef

define update
	$(cargo-update)
	$(rust-update)
endef

define upgrade
	$(cargo-upgrade)
endef

aws-list-profiles:
	aws configure list-profiles
	
aws-list-profile-jenkins-eks:
	aws configure list --profile jenkins-cluster
	
aws-configure:
	aws configure --profile jenkins-cluster
	
aws-describe-cluster:
	aws eks describe-cluster --name jenkins-agent-eks-cluster --query 'cluster.status'
	
aws-update-kubeconfig:
	aws eks update-kubeconfig --name jenkins-agent-eks-cluster
	
kubectx:
	kubectx arn:aws:eks:ap-southeast-1:473154593366:cluster/jenkins-eks-cluster
	
kubectl-get-pods:
	kubectl get pods
	
kubectl-get-pods-jenkins:
	kubectl get pods -n jenkins
	
kubectl-logs-jenkins-agent:
	kubectl logs jenkins-agent-deployment-68f4c46fff-mvsms -n jenkins
	
kubectl-apply-jenkins-agent:
	kubectl create namespace jenkins
	kubectl create secret generic jenkins-agent-secret --from-literal=jenkins-agent-secret=jenkins-agent-secret
	kubectl apply -f deployments/jenkins-agent/jenkins-agent-deployment.yaml

adr-init:
	adr init docs/adr

brew-install:
	brew install go
	brew install zstd
	brew install openssl@1.1
	brew install openssl@3
	brew install osv-scanner
	brew install wget

cargo-add:
	cargo add <library>

cargo-bench:
	cargo bench
	
cargo-install-diesel:
	cargo install diesel_cli

cargo-nightly:
	@rustup run nightly cargo udeps

cargo-profiler:
	cargo profiler cpu TARGET/release/rustdoc

cargo-test:
	cargo test

check-linux-version:
	cat /etc/os-release

cargo-check:
	$(cargo-check)
	
cargo-clean:
	cargo clean

cargo-update:
	$(cargo-update)

clear:
	$(clear)

compress:
	upx --best --lzma TARGET/release/rust
	
docker-build:
	$(docker-build)
	$(docker-push)

docker-push:
	$(docker-push)

docker-build-rust-asset:
	$(docker-build-rust-asset)

docker-build-rust-auth:
	$(docker-build-rust-auth)

docker-build-rust-consumer:
	$(docker-build-rust-consumer)

docker-build-rust-migration:
	$(docker-build-rust-migration)

docker-build-rust-oauth:
	$(docker-build-rust-oauth)

docker-build-rust-producer:
	$(docker-build-rust-producer)

docker-build-rust-scheduler:
	$(docker-build-rust-scheduler)

docker-build-rust-stock:
	$(docker-build-rust-stock)

docker-build-rust-user:
	$(docker-build-rust-user)

docker-build-rust-uuid:
	$(docker-build-rust-uuid)

docker-build-rust-web:
	$(docker-build-rust-web)

# Project Management
# =======================================================
docker-build-rust-employee:
	$(docker-build-rust-employee)

docker-build-rust-mail:
	$(docker-build-rust-mail)

docker-build-rust-project:
	$(docker-build-rust-project)

docker-build-rust-project-management:
	$(docker-build-rust-project-management)
# =======================================================
# End - Project Management	

docker-push-rust-alpine:
	$(docker-push-rust-alpine)
	
docker-push-rust-asset:
	$(docker-push-rust-asset)

docker-push-rust-auth:
	$(docker-push-rust-auth)

docker-push-rust-migration:
	$(docker-push-rust-migration)

docker-push-rust-oauth:
	$(docker-push-rust-oauth)

docker-push-rust-stock:
	$(docker-push-rust-stock)

docker-push-rust-uuid:
	$(docker-push-rust-uuid)

docker-push-rust-web:
	$(docker-push-rust-web)

docker-rm-rust-stock:
	$(docker-rm-rust-stock)

docker-rm-rust-web:
	$(docker-rm-rust-web)

docker-connect-postgres:
	docker exec -it 70d371631091 psql -U aditya.kristianto -d postgres

docker-container-stop:
	$(docker-container-stop)

docker-login:
	docker login -u ${DOCKER_USERNAME}

docker-remove:
	$(docker-remove)

docker-remove-asset:
	$(docker-remove-asset)

docker-remove-auth:
	$(docker-remove-auth)

docker-remove-oauth:
	$(docker-remove-oauth)

docker-remove-stock:
	$(docker-remove-stock)

docker-remove-uuid:
	$(docker-remove-uuid)

docker-remove-web:
	$(docker-remove-web)

docker-start:
	$(docker-start)
	
chmod-400:
	chmod 400 "infra/arm64/jenkins-eks/modules/key_pair/my_generated_key.pem"
	chmod 400 "infra/arm64/jenkins-eks/modules/key_pair/bastion-key.pem"

ssh:
	# ssh -i "infra/arm64/jenkins-eks/modules/key_management_service/my_generated_key.pem" ec2-user@10.0.101.147
	ssh -i "infra/arm64/jenkins-eks/modules/key_pair/bastion-key.pem" ec2-user@ec2-46-137-201-52.ap-southeast-1.compute.amazonaws.com

cloudbuild-docker-build:
	$(cloudbuild-docker-build)
	
cloudbuild-docker-alpine-build:
	./scripts/docker.sh ${build_dir_alpine_name} "" $(image_name_alpine) "" "" "" $(DOCKER_IMAGE_VERSION) $(TARGET) "" "" "" ${AWS_REGION} ${AWS_ACCOUNT_ID} ${VERBOSE_OUTPUT} true

cloudbuild-docker-asset-build:
	./scripts/docker-asset.sh ${build_dir_asset_name} $(image_name_asset_service) ${HOST_PORT} $(container_http_port_asset_service) $(container_TCP_PORT_asset_service) $(DOCKER_IMAGE_VERSION) $(TARGET) $(TEMPLATE_TYPE) ${IS_ECR} ${AWS_REGION} ${AWS_ACCOUNT_ID}

cloudbuild-docker-auth-build:
	./scripts/docker.sh ${build_dir_app_name} ${bin_auth_service} $(image_name_auth_service) $(HOST_PORT) $(CONTAINER_HTTP_PORT_AUTH_SERVICE) $(CONTAINER_TCP_PORT_AUTH_SERVICE) $(DOCKER_IMAGE_VERSION) $(TARGET) $(TEMPLATE_TYPE) false ${IS_ECR} ${AWS_REGION} ${AWS_ACCOUNT_ID} ${VERBOSE_OUTPUT}

cloudbuild-docker-oauth-build:
	./scripts/docker.sh ${build_dir_app_name} ${bin_oauth_service} $(image_name_oauth_service) $(HOST_PORT) $(container_http_port_oauth_service) $(container_TCP_PORT_oauth_service) $(DOCKER_IMAGE_VERSION) $(TARGET) $(TEMPLATE_TYPE) false ${IS_ECR} ${AWS_REGION} ${AWS_ACCOUNT_ID} ${VERBOSE_OUTPUT}

cloudbuild-docker-stock-build:
	./scripts/docker.sh ${build_dir_app_name} ${bin_stock_service} $(image_name_stock_service) $(HOST_PORT) $(container_http_port_stock_service) $(container_TCP_PORT_stock_service) $(DOCKER_IMAGE_VERSION) $(TARGET) $(TEMPLATE_TYPE) false ${IS_ECR} ${AWS_REGION} ${AWS_ACCOUNT_ID} ${VERBOSE_OUTPUT}

cloudbuild-docker-ubuntu-build:
	./scripts/docker.sh ${build_dir_ubuntu_name} "" $(image_name_ubuntu) "" "" "" $(DOCKER_IMAGE_VERSION) $(TARGET) "" "" "" ${AWS_REGION} ${AWS_ACCOUNT_ID} ${VERBOSE_OUTPUT} true

cloudbuild-docker-uuid-build:
	./scripts/docker.sh ${build_dir_app_name} ${bin_uuid_service} $(image_name_uuid_service) $(HOST_PORT) $(container_http_port_uuid_service) $(container_TCP_PORT_uuid_service) $(DOCKER_IMAGE_VERSION) $(TARGET) $(TEMPLATE_TYPE) false ${IS_ECR} ${AWS_REGION} ${AWS_ACCOUNT_ID} ${VERBOSE_OUTPUT}

cloudbuild-docker-web-build:
	./scripts/docker.sh ${build_dir_app_name} ${bin_web_service} $(image_name_web_service) $(HOST_PORT) $(container_http_port_web_service) $(container_TCP_PORT_web_service) $(DOCKER_IMAGE_VERSION) $(TARGET) $(TEMPLATE_TYPE) false ${IS_ECR} ${AWS_REGION} ${AWS_ACCOUNT_ID} ${VERBOSE_OUTPUT}

cloudbuild-docker-push:
	$(cloudbuild-docker-push)

cloudbuild-docker-auth-push:
	$(cloudbuild-docker-auth-push)

cloudbuild-docker-oauth-push:
	$(cloudbuild-docker-oauth-push)

cloudbuild-docker-stock-push:
	$(cloudbuild-docker-stock-push)

cloudbuild-docker-uuid-push:
	$(cloudbuild-docker-uuid-push)

cloudbuild-docker-web-push:
	$(cloudbuild-docker-web-push)

cloudbuild-docker-asset-push:
	$(cloudbuild-docker-asset-push)

cloudbuild-ecr-remove-alpine-images:
	./scripts/ecr-remove-image.sh ${AWS_REGION} ${image_name_alpine} ${VERSIONS_TO_RETAIN}

cloudbuild-ecr-remove-auth-images:
	./scripts/ecr-remove-image.sh ${AWS_REGION} ${image_name_auth_service} ${VERSIONS_TO_RETAIN}

cloudbuild-ecr-remove-asset-images:
	./scripts/ecr-remove-image.sh ${AWS_REGION} ${image_name_asset_service} ${VERSIONS_TO_RETAIN}

cloudbuild-ecr-remove-oauth-images:
	./scripts/ecr-remove-image.sh ${AWS_REGION} ${image_name_oauth_service} ${VERSIONS_TO_RETAIN}

cloudbuild-ecr-remove-stock-images:
	./scripts/ecr-remove-image.sh ${AWS_REGION} ${image_name_stock_service} ${VERSIONS_TO_RETAIN}

cloudbuild-ecr-remove-ubuntu-images:
	./scripts/ecr-remove-image.sh ${AWS_REGION} ${image_name_ubuntu} ${VERSIONS_TO_RETAIN}

cloudbuild-ecr-remove-uuid-images:
	./scripts/ecr-remove-image.sh ${AWS_REGION} ${image_name_uuid_service} ${VERSIONS_TO_RETAIN}

cloudbuild-ecr-remove-web-images:
	./scripts/ecr-remove-image.sh ${AWS_REGION} ${image_name_web_service} ${VERSIONS_TO_RETAIN}
	
clean-all:
	sudo yum clean all
	
disk-free:
	df -h

docker-build-gitlab:
	docker build -f build/ci/gitlab/Dockerfile -t gitlab:latest .

docker-build-rust-alpine:
	$(docker-build-rust-alpine)

docker-buildx-install:
	docker buildx install

docker-buildx-uninstall:
	docker buildx uninstall

docker-buildx-build:
	$(docker-buildx-build)

docker-buildx-build-using-cloudbuild:
	$(docker-buildx-build-using-cloudbuild)

docker-buildx-version:
	docker buildx version

docker-image-prune:
	docker image prune

docker-inspect:
	# docker inspect <container-id> | grep "IPAddress"
	./scripts/docker-inspect.sh

docker-run:
	$(docker-run)
	# $(docker-run-mongodb)
	# $(docker-run-redis)
	# $(docker-run-redisinsight)
	# $(docker-run-rust-app)

docker-run-rust-asset:
	$(docker-run-rust-asset)

docker-run-rust-auth:
	$(docker-run-rust-auth)

docker-run-rust-oauth:
	$(docker-run-rust-oauth)

docker-run-rust-stock:
	$(docker-run-rust-stock)

docker-run-rust-uuid:
	$(docker-run-rust-uuid)

docker-run-rust-web:
	$(docker-run-rust-web)

# Project Management
# =======================================================
docker-run-rust-employee:
	$(docker-run-rust-employee)

docker-run-rust-mail:
	$(docker-run-rust-mail)

docker-run-rust-project:
	$(docker-run-rust-project)

docker-run-rust-project-management:
	$(docker-run-rust-project-management)
# =======================================================
# End - Project Management

docker-run-alpine:
	docker run --rm public.ecr.aws/docker/library/rust:1.77.0-alpine3.19 sh -c "apk add --no-cache gcc && gcc --version"

docker-run-kong:
	$(docker-run-kong)

docker-run-mongodb:
	docker run -d -p 27017:27017 --name mongodb -e MONGO_INITDB_ROOT_USERNAME=root -e MONGO_INITDB_ROOT_PASSWORD=password mongo:latest

docker-run-mongodb-community-server:
	$(docker-run-mongodb-community-server)

docker-run-jenkins:
	$(docker-run-jenkins)

docker-run-portainer:
	$(docker-run-portainer)
	
docker-run-postgres:
	$(docker-run-postgres)

docker-run-pgadmin:
	$(docker-run-pgadmin)

docker-run-rabbitmq:
	$(docker-run-rabbitmq)

docker-run-redis:
	$(docker-run-redis)
	
docker-run-redisinsight:
	$(docker-run-redisinsight)

docker-run-sonarqube:
	docker volume create --name sonarqube_data
	docker volume create --name sonarqube_logs
	docker volume create --name sonarqube_extensions
	docker run -d \
    	-p 9000:9000 \
		--name sonarqube \
    	-v sonarqube_data:/opt/sonarqube/data \
    	-v sonarqube_extensions:/opt/sonarqube/extensions \
    	-v sonarqube_logs:/opt/sonarqube/logs \
    	sonarqube:latest

docker-run-rust-app:
#	$(docker-build)
#	docker stop rust-app
#	docker rm rust-app
#	docker run --name rust-app -d -p 8080:8080 $(app_name):$(DOCKER_IMAGE_VERSION)-$(TARGET)
#	docker run --name rust-app -d -p 8080:8080 473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/rust:v5.15-release
# ./scripts/docker.sh $(bin_1) $(app_name) $(image_name) $(container_name) $(HOST_PORT) $(container_port) $(DOCKER_IMAGE_VERSION) $(TARGET) $(TEMPLATE_TYPE)
# ./scripts/docker.sh $(bin_2) $(app_name) $(image_name) $(container_name) $(HOST_PORT) $(container_port) $(DOCKER_IMAGE_VERSION) $(TARGET)
# ./scripts/docker.sh "" $(app_name_asset) $(image_name_asset) $(container_name_asset) $(HOST_PORT) $(container_port_asset) $(DOCKER_IMAGE_VERSION) $(TARGET) $(TEMPLATE_TYPE) true
# ./scripts/docker.sh ${bin_auth_service} $(app_name) $(image_name_auth_service) $(container_name_auth_service) $(HOST_PORT) $(CONTAINER_TCP_PORT_AUTH_SERVICE) $(DOCKER_IMAGE_VERSION) $(TARGET) $(TEMPLATE_TYPE) false ${IS_ECR} ${AWS_REGION}
# ./scripts/docker.sh $(bin_stock_service) $(app_name) $(image_name_stock_service) $(container_name_stock_service) $(HOST_PORT) $(container_port_stock_service) $(DOCKER_IMAGE_VERSION) $(TARGET) $(TEMPLATE_TYPE) ${IS_ECR} ${AWS_REGION}
	#docker build -t public-assets-container .
	#docker run -d -p 8080:80 public-assets-container
	aws ecr get-login-password --region ap-southeast-1 | docker login --username AWS --password-stdin 473154593366.dkr.ecr.ap-southeast-1.amazonaws.com
	docker run --name ${bin_asset_service} -d -p ${container_port_asset_service}:${HOST_PORT} 473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/${image_name_asset_service}:1.0.31-${TARGET}
	docker run --name ${bin_auth_service} -d -p ${CONTAINER_TCP_PORT_AUTH_SERVICE}:${HOST_PORT} 473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/${image_name_auth_service}:1.0.29-${TARGET}
	docker run --name ${bin_oauth_service} -d -p ${container_port_oauth_service}:${HOST_PORT} 473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/${image_name_oauth_service}:1.0.12-${TARGET}
	docker run --name ${bin_stock_service} -d -p ${container_port_stock_service}:${HOST_PORT} 473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/${image_name_stock_service}:1.0.12-${TARGET}
	docker run --name ${bin_uuid_service} -d -p ${container_port_uuid_service}:${HOST_PORT} 473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/${image_name_uuid_service}:1.0.12-${TARGET}
	docker run --name ${bin_web_service} -d -p ${container_port_web_service}:${HOST_PORT} 473154593366.dkr.ecr.ap-southeast-1.amazonaws.com/${image_name_web_service}:1.0.25-${TARGET}

docker-run-tyk:
	$(docker-run-tyk)

docker-run-tyk-dashboard:
	docker run -d --name tyk_dashboard \
  		-p 3000:3000 \
		-e REDIS_HOST=172.17.0.5 \
		-e REDIS_PASSWORD= \
  		-e TYK_DB_CONFIG_PATH=tyk-dashboard/tyk_analytics.conf \
  		-e TYK_SECRET=tyk-dashboard/tyk_analytics.secret \
		-e TYK_IB_SESSION_SECRET=${PG_PASSWORD} \
		-e TYK_DB_STORAGE_MAIN_TYPE=postgres \
		-e "TYK_DB_STORAGE_MAIN_CONNECTIONSTRING=user=aditya.kristianto password=${PG_PASSWORD} host=172.17.0.3 port=5432 database=tyk_analytics" \
		tykio/tyk-dashboard:v5.2

docker-scout-quickview:
	docker scout quickview

docker-system-prune:
	docker system prune

docker-compose-up-kafka:
	docker-compose -f build/ci/kafka/development/docker-compose.yml -p kafka up -d

docker-compose-down-kafka:
	docker-compose -f build/ci/kafka/development/docker-compose.yml down -d

docker-run-lenses:
	docker run -e ADV_HOST=127.0.0.1 \
       -e EULA="https://licenses.lenses.io/d/?id=271969c0-45b1-11ee-8f1e-42010af01003" \
       --rm -p 3030:3030 -p 9092:9092 lensesio/box:latest

docker-run-elasticsearch:
	$(docker-run-elasticsearch)

docker-run-kafka:
	$(docker-run-kafka)

docker-run-kafka-exporter:
	$(docker-run-kafka-exporter)

docker-run-kafka-ui:
	$(docker-run-kafka-ui)

docker-version:
	docker --version

build:
	cargo build --jobs 4

build-release:
	cargo build --release --jobs 4
	
build-auth-release:
	$(generate-templates)
	cargo build --bin auth --release
	
build-consumer-release:
	$(generate-templates)
	cargo build --bin consumer --release
	
build-oauth-release:
	$(generate-templates)
	cargo build --bin oauth --release
	
build-producer-release:
	$(generate-templates)
	cargo build --bin producer --release
	
build-rust-release:
	$(generate-templates)
	cargo build --bin rust --release
	
build-stock-release:
	$(generate-templates)
	cargo build --bin stock --release
	
build-user-release:
	$(generate-templates)
	cargo build --bin user --release
	
build-uuid-release:
	$(generate-templates)
	cargo build --bin uuid --release
	
build-web-release:
	$(generate-templates)
	cargo build --bin web --release

diesel-migration-generate:
	diesel migration generate ... --migration-dir ...
	# diesel migration generate create_transactions_table --migration-dir db/migrations/stock

diesel-redo:
	diesel migration redo --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/rust_app"

diesel-run:
	diesel migration run --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/rust_app"

# Project Management
# =======================================================
diesel-employee-redo:
	diesel migration revert --migration-dir db/migrations/employee --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/employee"

diesel-employee-revert:
	diesel migration revert --migration-dir db/migrations/employee --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/employee"

diesel-employee-revert-all:
	diesel migration revert --all --migration-dir db/migrations/employee --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/employee"

diesel-employee-run:
	diesel migration run --migration-dir db/migrations/employee --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/employee"

diesel-mail-redo:
	diesel migration revert --migration-dir db/migrations/mail --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/mail"

diesel-mail-revert:
	diesel migration revert --migration-dir db/migrations/mail --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/mail"

diesel-mail-revert-all:
	diesel migration revert --all --migration-dir db/migrations/mail --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/mail"

diesel-mail-run:
	diesel migration run --migration-dir db/migrations/mail --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/mail"

diesel-project-redo:
	diesel migration revert --migration-dir db/migrations/project --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/project"

diesel-project-revert:
	diesel migration revert --migration-dir db/migrations/project --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/project"

diesel-project-revert-all:
	diesel migration revert --all --migration-dir db/migrations/project --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/project"
# =======================================================
# End - Project Management

diesel-project-run:
	diesel migration run --migration-dir db/migrations/project --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/project"

diesel-stock-run:
	diesel migration run --migration-dir db/migrations/stock --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/rust_app"

diesel-revert:
	diesel migration revert --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/rust_app"

diesel-revert-all:
	diesel migration revert --all --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/rust_app"
	
diesel-migration-list:
	diesel migration list --migration-dir db/migrations/employee --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/rust_app"

diesel-print-schema:
	diesel print-schema --database-url "postgres://${PG_USER}:${PG_PASSWORD}@localhost:5432/rust_app"

fix:
	cargo fix --bin "rust" --allow-dirty

git-config:
	git config --global user.email "kristianto.aditya@gmail.com"
	git config --global user.name "Aditya Kristianto"
	git config pull.rebase false

git-push-mirror:
	git push --mirror ssh://git-codecommit.ap-southeast-1.amazonaws.com/v1/repos/RustApp
		
host:
	rustc -vV
	
html-minifier:
	html-minifier --collapse-whitespace --remove-comments --remove-redundant-attributes --remove-script-type-attributes --remove-tag-whitespace --minify-js true static/${TEMPLATE_TYPE}/dist/index.html -o static/${TEMPLATE_TYPE}/dist/index.html
	
minify-install:
	npm install -g uglify-js
	npm install -g terser

minify:
	$(clear)
	$(minify)
	
install:
	# install rust 
	curl https://sh.rustup.rs -sSf | sh
	source "$HOME/.cargo/env"
	yum install upx
	rustup toolchain install nightly
	rustup component add rust-src --toolchain nightly
	# install docker-compose
	sudo curl -L https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m) -o /usr/local/bin/docker-compose
	sudo chmod +x /usr/local/bin/docker-compose
	docker-compose version
	
install-cargo:
	curl https://sh.rustup.rs -sSf | sh
	
install-cmake:
	sudo yum install cmake
	
install-docker:
	brew install --cask docker
	
install-mysql:
	sudo rpm -Uvh https://repo.mysql.com/mysql80-community-release-el7-3.noarch.rpm
	sudo yum install openssl openssl-devel pkgconfig mysql-community-client mysql-community-devel --skip-broken
	
install-postgres:
	sudo yum install postgresql-devel

install-npm:
	sudo apt install npm

install-log4brains:
	npm install -g log4brains

install-rust:
	brew install rust

install-visual-studio-code:
	brew install --cask visual-studio-code
	
install-webp:
	sudo yum update -y
	sudo yum install -y libwebp-tools
	cwebp -version

kafka-extract:
	tar -xzf kafka_2.13-3.5.0.tgz

kafka-start:
	bin/zookeeper-server-start.sh config/zookeeper.properties
	bin/kafka-server-start.sh config/server.properties

kafka-create-topic:
	bin/kafka-topics.sh --create --topic quickstart-events --bootstrap-server localhost:9092

kafka-write-event:
	bin/kafka-console-producer.sh --topic quickstart-events --bootstrap-server localhost:9092

kafka-read-event:
	bin/kafka-console-consumer.sh --topic quickstart-events --from-beginning --bootstrap-server localhost:9092

kubectl-apply:
	$(kubectl-apply)

kubectl-delete-deployment:
	kubectl delete deployment <deployment-name>

kubectl-edit-pod:
	kubectl edit pod <pod-name>

kubectl-get-all:
	kubectl get all

kubectl-get-deployments:
	kubectl get deployments

kubectl-get-namespace:
	kubectl get namespace

kubectl-get-pod:
	kubectl get pods

kubectl-get-service:
	kubectl get service
	
kubectl-install:
	sudo yum update -y
	sudo yum install -y epel
	sudo yum install -y kubectl
	kubectl version --client
	# curl -LO "https://dl.k8s.io/release/$(curl -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
	# chmod +x ./kubectl
	# sudo mv ./kubectl /usr/local/bin/kubectl
	# kubectl version --client

kubectl-logs:
	kubectl logs <pod-name>

kubectl-run:
	$(docker-build)
	$(kubectl-apply)

log4brains-init:
	log4brains init

log4brains-preview:
	log4brains preview

log4brains-adr-new:
	log4brains adr new

log4brains-build:
	log4brains build

nightly-build:
	# cargo +nightly build -Z build-std=std,panic_abort --TARGET x86_64-unknown-linux-gnu --release
	cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
    --TARGET x86_64-unknown-linux-gnu --release

open-docker:
	open /Applications/Docker.app

osv-scanner:
	osv-scanner -r ./

publish:
	cargo publish

# To run the rust app directly
# ================================================================================================================================
run:
	cargo run

run-duuid-release:
	cargo run --bin duuid --release

run-auth:
	RUST_LOG=debug cargo run --bin auth

run-auth-release:
	RUST_BACKTRACE=full cargo run --bin auth --release

run-consumer:
	RUST_LOG=debug cargo run --bin consumer

run-oauth:
	cargo run --bin oauth

run-rust:
	cargo run --bin rust

run-user:
	RUST_LOG=debug cargo run --bin user

run-rust-release:
	cargo run --bin rust --release

run-stock-amd64:
	cargo run --bin stock --target x86_64-apple-darwin

run-stock-arm64:
	cargo run --bin stock --target aarch64-apple-darwin

run-uuid:
	cargo run --bin uuid

run-release:
	cargo run --release
	
run-consumer-release:
	RUST_BACKTRACE=full cargo run --bin consumer --release
	
run-producer-release:
	RUST_BACKTRACE=full cargo run --bin producer --release

run-stock-release:
	RUST_BACKTRACE=full cargo run --bin stock --release
	
run-user-release:
	RUST_BACKTRACE=full cargo run --bin user --release

run-uuid-release:
	RUST_BACKTRACE=full cargo run --bin uuid --release

run-tyk:
	docker network create tyk
	docker run -itd --rm --name redis --network tyk -p 127.0.0.1:6379:6379 redis:4.0-alpine
	wget https://raw.githubusercontent.com/TykTechnologies/tyk-gateway-docker/master/tyk.standalone.conf
	docker run \
      --name tyk_gateway \
      --network tyk \
      -p 8080:8080 \
      -v $(pwd)/tyk.standalone.conf:/opt/tyk-gateway/tyk.conf \
      -v $(pwd)/apps:/opt/tyk-gateway/apps \
      docker.tyk.io/tyk-gateway/tyk-gateway:latest
	
run-uuid-debug:
	cargo run --bin uuid

run-web:
	$(docker-build-rust-asset)
	$(docker-run-rust-asset)
	cargo run --bin web

run-web-release:
	$(generate-templates)
	$(docker-build-rust-asset)
	$(docker-run-rust-asset)
	RUST_BACKTRACE=full cargo run --bin web --release

# ================================================================================================================================

rust-update:
	$(rustup-update)
	
rust-version:
	rustc --version

s3-bucket:
	aws s3api create-bucket --bucket rust-bucket --region ap-southeast-1 --create-bucket-configuration LocationConstraint=ap-southeast-1

s3-check-bucket:
	aws s3api head-bucket --bucket "rust-bucket"
	
s3-bucket-versioning:
	aws s3api put-bucket-versioning --bucket rust-bucket --versioning-configuration Status=Enabled

update:
	$(update)

upgrade:
	$(upgrade)

create-s3-bucket:
	aws s3api create-bucket --bucket kubernetes-aws-wthamira-io --create-bucket-configuration LocationConstraint=eu-west-1

copy-to-s3-bucket:
	aws s3 cp image.png s3://aditya-kristianto-bucket/ --region ap-southeast-1

terraform-ami:
	aws ec2 describe-images --owners amazon --filters "Name=name,Values=amzn2-ami-hvm-*" --region ap-southeast-1

terraform-apply:
	cd infra/arm64 && terraform apply -auto-approve
	
terraform-db-apply:
	cd infra/arm64/db && terraform apply -auto-approve
	
terraform-jenkins-apply:
	cd infra/arm64/jenkins && terraform apply -auto-approve

terraform-jenkins-eks-apply:
	cd infra/arm64/jenkins-eks && terraform apply -auto-approve
	
terraform-jenkins-eks-apply-first:
	cd infra/arm64/jenkins-eks && terraform apply -target=data.aws_eks_node_groups.jenkins_node_groups -auto-approve
	
terraform-jenkins-eks-apply-second:
	cd infra/arm64/jenkins-eks && terraform apply -auto-approve
	
terraform-test-apply:
	cd infra/arm64/test && terraform apply -auto-approve
	
terraform-destroy:
	cd infra/arm64 && terraform destroy -auto-approve
	
terraform-db-destroy:
	cd infra/arm64/db && terraform destroy -auto-approve
	
terraform-jenkins-destroy:
	cd infra/arm64/jenkins && terraform destroy -auto-approve
	
terraform-jenkins-eks-destroy:
	cd infra/arm64/jenkins-eks && terraform destroy -auto-approve
	
terraform-test-destroy:
	cd infra/arm64/test && terraform destroy -auto-approve
	
terraform-health:
	aws eks describe-cluster --name arm64-eks-cluster
	
terraform-db-health:
	aws eks describe-cluster --name arm64-eks-cluster
	
terraform-iam:
	aws sts get-caller-identity
	
terraform-init:
	cd infra/arm64 && terraform init
	
terraform-db-init:
	cd infra/arm64/db && terraform init
	
terraform-jenkins-init:
	cd infra/arm64/jenkins && terraform init
	
terraform-jenkins-eks-init:
	cd infra/arm64/jenkins-eks && terraform init
	
terraform-jenkins-eks-init-upgrade:
	cd infra/arm64/jenkins-eks && terraform init -upgrade
	
terraform-test-init:
	cd infra/arm64/test && terraform init
	
terraform-test-init-upgrade:
	cd infra/arm64/test && terraform init -upgrade
	
terraform-install:
	sudo yum update -y
	sudo yum install -y yum-utils
	sudo yum-config-manager --add-repo https://rpm.releases.hashicorp.com/AmazonLinux/hashicorp.repo
	sudo yum install -y terraform
	terraform -v

terraform-plan:
	cd infra/arm64 && terraform plan
	
terraform-db-plan:
	cd infra/arm64/db && terraform plan

terraform-jenkins-eks-plan:
	cd infra/arm64/jenkins-eks && terraform plan
	
terraform-db-ssm:
	aws ssm start-session --target i-0c493fa7831ff02fb
	
terraform-validate:
	cd infra/arm64 && terraform validate
	
terraform-db-validate:
	cd infra/arm64/db && terraform validate
	
terraform-jenkins-validate:
	cd infra/arm64/jenkins && terraform validate
	
terraform-jenkins-eks-validate:
	cd infra/arm64/jenkins-eks && terraform validate
	
terraform-test-validate:
	cd infra/arm64/test && terraform validate

upload-photo-via-api-gateway:
	curl --location --request PUT 'https://5ajbtqmuq1.execute-api.ap-southeast-1.amazonaws.com/development/s3' \
		--header 'Content-Type: image/jpeg' \
		--data '@/Users/adityakristianto/Pictures/2023_09_26 AWS Cloud Day/DE2A0001.jpg'
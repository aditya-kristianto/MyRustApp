# Use a build argument for the base image
ARG BASE_IMAGE

# Stage 1: Build Application
# Use the specified base image
FROM $BASE_IMAGE as builder 

ARG BIN
ARG TARGET
ARG VERBOSE_OUTPUT
ARG TEMPLATE_TYPE
ARG ASSET_NAME
ARG ASSET_VERSION

RUN ARCH=$(uname -m) && \ 
    if [ "$ARCH" == "aarch64" ]; then \
        if [ -z "$BIN" ]; then \
                echo Argument Bin not provided; \
                exit 1; \
        fi && \
        if [ -z "$TARGET" ]; then \
                echo Argument Target not provided; \
                exit 1; \
        fi && \
        if [ -z "$VERBOSE_OUTPUT" ]; then \
                echo Argument Verbose Ouput not provided; \
                exit 1; \
        fi && \
        if [ -z "$TEMPLATE_TYPE" ]; then \
                echo Argument Template Type not provided; \
                exit 1; \
        fi \
    fi

WORKDIR /usr/src/myapp

RUN echo "Rust Version: $(rustc --version)"
RUN echo "Cargo Version: $(cargo --version)"

RUN mkdir -p pkg
RUN mkdir -p static
RUN mkdir -p src
RUN mkdir -p templates/authentication/general
RUN mkdir -p templates/authentication/layouts/corporate
RUN mkdir -p templates/pages/pricing
RUN mkdir -p db/migrations/${BIN}

COPY .env .
COPY Cargo.toml .
COPY pkg pkg
COPY src src
COPY static/robots.txt static/robots.txt
COPY static/${ASSET_NAME}/${ASSET_VERSION}/${TEMPLATE_TYPE}/index.min.html templates
COPY static/${ASSET_NAME}/${ASSET_VERSION}/${TEMPLATE_TYPE}/authentication/general/coming-soon.min.html templates/authentication/general
COPY static/${ASSET_NAME}/${ASSET_VERSION}/${TEMPLATE_TYPE}/authentication/general/error-404.min.html templates/authentication/general
COPY static/${ASSET_NAME}/${ASSET_VERSION}/${TEMPLATE_TYPE}/authentication/general/error-500.min.html templates/authentication/general
COPY static/${ASSET_NAME}/${ASSET_VERSION}/${TEMPLATE_TYPE}/authentication/general/verify-email.min.html templates/authentication/general
COPY static/${ASSET_NAME}/${ASSET_VERSION}/${TEMPLATE_TYPE}/authentication/general/welcome.min.html templates/authentication/general
COPY static/${ASSET_NAME}/${ASSET_VERSION}/${TEMPLATE_TYPE}/authentication/layouts/corporate/sign-in.min.html templates/authentication/layouts/corporate
COPY static/${ASSET_NAME}/${ASSET_VERSION}/${TEMPLATE_TYPE}/authentication/layouts/corporate/reset-password.min.html templates/authentication/layouts/corporate
COPY static/${ASSET_NAME}/${ASSET_VERSION}/${TEMPLATE_TYPE}/authentication/layouts/corporate/new-password.min.html templates/authentication/layouts/corporate
COPY static/${ASSET_NAME}/${ASSET_VERSION}/${TEMPLATE_TYPE}/pages/contact.min.html templates/pages
COPY static/${ASSET_NAME}/${ASSET_VERSION}/${TEMPLATE_TYPE}/pages/team.min.html templates/pages
COPY static/${ASSET_NAME}/${ASSET_VERSION}/${TEMPLATE_TYPE}/pages/pricing/table.min.html templates/pages/pricing
COPY db/migrations/${BIN} db/migrations/${BIN}

RUN cargo clean
RUN rustup update
RUN cargo update

# ✅ Identify the compiler
RUN ln -sf /usr/bin/aarch64-alpine-linux-musl-gcc /usr/bin/musl-gcc
RUN ls /usr/bin/*musl-gcc*
RUN which aarch64-alpine-linux-musl-gcc

# ✅ Build the application
RUN if [ "$VERBOSE_OUTPUT" = "true" ]; then \
        if [[ ! -z "$TARGET" ]] && [ "$TARGET" == "release" ]; then \
            RUSTFLAGS="-C target-feature=-crt-static -C link-args=-s -L /usr/lib -lssl -lcrypto -lz" \
            cargo build --bin ${BIN} --release --jobs 8 -vv; \
        else \
            cargo build --bin ${BIN} --target=aarch64-unknown-linux-musl --jobs 8; \
        fi \
    else \
        if [[ ! -z "$TARGET" ]] && [ "$TARGET" == "release" ]; then \
            RUSTFLAGS='-C target-feature=-crt-static link-args=-s -L /usr/lib -l ssl -l crypto -l z' \
            cargo build --bin ${BIN} --release --jobs 8; \
        else \
            cargo build --bin ${BIN} --target=aarch64-unknown-linux-musl --jobs 8; \
        fi \
    fi

# RUN if [[ ! -z "$TARGET" ]] && [ "$TARGET" != "release" ]; then upx -d --best --lzma target/${target}/rust; fi

# =============================================================================
# Stage 2: Final Image
FROM public.ecr.aws/docker/library/alpine:3.21.3

ARG BIN
ARG TARGET
ARG TEMPLATE_TYPE
ARG EXPOSE_HTTP_PORT
ARG EXPOSE_TCP_PORT
ARG APP_VERSION

WORKDIR /usr/local/bin

# ✅ Install required dependencies
RUN apk add --no-cache \
    libgcc

COPY --from=builder /usr/src/myapp/.env .env
COPY --from=builder /usr/src/myapp/target/${TARGET}/${BIN} myapp
COPY --from=builder /usr/src/myapp/static static
COPY --from=builder /usr/src/myapp/templates templates

RUN sed -i "s/^APP_VERSION=.*/APP_VERSION=${APP_VERSION}/" .env

EXPOSE ${EXPOSE_HTTP_PORT}
EXPOSE ${EXPOSE_TCP_PORT}/tcp

ENTRYPOINT ["myapp"]

# Define a health check that sends an HTTP request to the service's endpoint
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD wget --header='accept: application/json' --no-check-certificate -q --spider -O /dev/null http://localhost:${EXPOSE_HTTP_PORT}/healthcheck || exit 1

# Define a health check that checks if a TCP port is listening
# HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
#     CMD nc -z -w 1 127.0.0.1 ${EXPOSE_TCP_PORT} || exit 1

# Define a health check that runs a custom script
# HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 CMD /path/to/custom-health-check.sh || exit 1
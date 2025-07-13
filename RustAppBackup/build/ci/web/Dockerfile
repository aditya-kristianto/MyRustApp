FROM public.ecr.aws/docker/library/rust:1.77.0-alpine3.19 as builder

ARG bin
ARG target
ARG template_type
ARG ASSET_NAME
ARG ASSET_VERSION

RUN if [ -z "$bin" ]; then \
      echo Argument Bin not provided; \
      exit 1; \
    fi
RUN if [ -z "$target" ]; then \
      echo Argument Target not provided; \
      exit 1; \
    fi
RUN if [ -z "template_type" ]; then \
      echo Template Type not provided; \
      exit 1; \
    fi

WORKDIR /usr/src/myapp

RUN echo "Rust Version: $(rustc --version)"
RUN echo "Cargo Version: $(cargo --version)"

RUN mkdir -p pkg
RUN mkdir -p src

COPY .env .
COPY Cargo.toml .
COPY pkg pkg
COPY src src

RUN apk add --no-cache \
    bash \
    build-base \
    clang \
    clang-dev \
    cmake \
    gcc \
    g++ \ 
    libc-dev \
    libtool \
    openssl-dev \
    python3 \
    rapidjson \
    upx \
    zlib-dev && \
    rm -rf /var/cache/apk/*

# If needed, create symbolic links for library naming convention
RUN ls -al /usr/bin
# RUN ln -s /path/to/actual/libz.so.X.Y.Z /path/to/expected/libz.so

# Set LD_LIBRARY_PATH for runtime
ENV LD_LIBRARY_PATH="/usr/lib:${LD_LIBRARY_PATH}"

# Set LIBRARY_PATH for linking
ENV LIBRARY_PATH="/usr/lib:${LIBRARY_PATH}"

RUN export CMAKE_MAKE_PROGRAM=/usr/bin/make

RUN echo "gcc Version: $(gcc --version)"
RUN echo "g++ Version: $(g++ --version)"
RUN echo "cmake Version: $(cmake --version)"

RUN if [[ ! -z "$target" ]] && [ "$target" == "release" ]; then \
        RUSTFLAGS='-C link-args=-s' export CMAKE_MAKE_PROGRAM=/usr/bin/make && export CMAKE_C_COMPILER=/usr/bin/cc && cargo build --bin $bin --release --jobs 8; \
        else export CMAKE_MAKE_PROGRAM=/usr/bin/make && export CMAKE_C_COMPILER=/usr/bin/cc && cargo build --bin $bin --jobs 8; \
    fi

# RUN if [[ ! -z "$target" ]] && [ "$target" != "release" ]; then upx -d --best --lzma target/${target}/rust; fi

# =============================================================================

FROM public.ecr.aws/docker/library/alpine:3.19.1

ARG bin
ARG target
ARG template_type
ARG expose_port

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/myapp/.env .env
COPY --from=builder /usr/src/myapp/target/${target}/${bin} myapp
COPY /static/${ASSET_NAME}/${ASSET_VERSION}/${template_type}//dist static/${template_type}/dist

EXPOSE ${expose_port}/tcp

ENTRYPOINT ["myapp"]

HEALTHCHECK --interval=5s --timeout=3s CMD curl --fail http://localhost:8080/actuator/health || exit 1
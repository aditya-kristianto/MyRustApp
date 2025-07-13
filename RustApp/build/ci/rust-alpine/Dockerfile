FROM public.ecr.aws/docker/library/rust:1.85.0-alpine3.21 as builder

# ✅ Install required dependencies
RUN apk add --no-cache \
    bash \
    build-base \
    clang \
    clang-dev \
    cmake \
    curl \
    gcc \
    g++ \ 
    libc-dev \
    libgcc \
    libtool \
    make \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig \
    python3 \
    rapidjson \
    upx \
    wget \
    zlib-dev \
    zlib-static && \
    rm -rf /var/cache/apk/*

# If needed, create symbolic links for library naming convention
# RUN ls -al /usr/bin
# RUN ln -s /path/to/actual/libz.so.X.Y.Z /path/to/expected/libz.so

# Set LD_LIBRARY_PATH for runtime
ENV LD_LIBRARY_PATH="/usr/lib:/usr/local/lib:${LD_LIBRARY_PATH}"

# Set LIBRARY_PATH for linking
ENV LIBRARY_PATH="/usr/lib:/usr/local/lib:${LIBRARY_PATH}"

# ✅ Set correct linker
ENV CC_aarch64_unknown_linux_musl=musl-gcc
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc

# ✅ Set Rust flags to explicitly link zlib
ENV RUSTFLAGS="-C target-feature=-crt-static link-args=-s -L /usr/lib -l ssl -l crypto -l z"

# ✅ Ensure Rust has the correct target
RUN rustup target add aarch64-unknown-linux-musl

RUN export CMAKE_MAKE_PROGRAM=/usr/bin/make
RUN export CMAKE_C_COMPILER=/usr/bin/cc
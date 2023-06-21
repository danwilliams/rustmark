# syntax=docker/dockerfile:1.4

# BUILDER

FROM rust:latest as builder

# Install required packages
RUN <<EOF
    set -e
    apt-get update
    apt-get install -y \
        clang \
        musl-tools
    rustup target add x86_64-unknown-linux-musl
EOF

# Install build dependencies
WORKDIR /usr/src
RUN <<EOF
    set -e
    mkdir mold
    curl -SL https://github.com/rui314/mold/releases/download/v1.11.0/mold-1.11.0-x86_64-linux.tar.gz \
        | tar --strip-components=1 -xzC mold
    mv mold/bin/mold /usr/local/bin/
    rm -rf mold
    mkdir upx
    curl -SL https://github.com/upx/upx/releases/download/v4.0.2/upx-4.0.2-amd64_linux.tar.xz \
        | tar --strip-components=1 -xJC upx
    mv upx/upx /usr/local/bin/
    rm -rf upx
EOF

# Prepare build location
WORKDIR /usr/src
RUN <<EOF
    set -e
    mkdir -p rustmark/.cargo
    printf '
[target.x86_64-unknown-linux-musl]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/mold"]' \
        > rustmark/.cargo/config
EOF

# By default, this Dockerfile builds in release mode. To build for local dev,
# set the "profile" argument to "dev". This will be passed to cargo. The dev
# profile enables debug symbols and disables optimisations.
ARG profile=release

# The "cargo_opts" argument allows for passing additional options to cargo. This
# can be useful to override settings in the Cargo.toml file. For example, to
# change the optimisation level to 3 (the maximum), you can pass the following
# argument: --build-arg cargo_opts="--config opt-level=3"
ARG cargo_opts=""

# Initial non-project build to cache dependencies
WORKDIR /usr/src/rustmark
COPY Cargo.toml Cargo.lock ./
RUN <<EOF
    set -e
    mkdir src
    echo "fn main() {}" > src/main.rs
    cp src/main.rs build.rs
    cargo build --profile=$profile --target=x86_64-unknown-linux-musl $cargo_opts
    rm build.rs
    rm src/main.rs
    rmdir src
    target_path=/usr/src/rustmark/target/x86_64-unknown-linux-musl
    ln -s $target_path/debug $target_path/dev
EOF

# Copy source files and build project codebase, minus content
COPY src src
COPY html html
COPY static static
RUN <<EOF
    set -e
    mkdir content
    echo "fn main() {}" > build.rs
    touch src/main.rs
    cargo build --profile=$profile --target=x86_64-unknown-linux-musl $cargo_opts
    rm build.rs
    rmdir content
EOF

# Copy content files and build full project, including content
COPY build.rs ./
COPY content content
RUN <<EOF
    set -e
    touch build.rs
    cargo build --profile=$profile --target=x86_64-unknown-linux-musl $cargo_opts
EOF

# The "upx" argument can be set to 1 or 0 to enable or disable compression of
# the binary, which increases the build time but results in a smaller image. If
# the "dev" argument is enabled, then this argument is ignored, as compressing
# the binary would not make sense on a development build.
ARG upx=1

# Compress binary executable
RUN <<EOF
    set -e
    if [ "$upx" = "1" ] && [ "$profile" != "dev" ]; then
        upx --best target/x86_64-unknown-linux-musl/$profile/rustmark
    fi
EOF


# RUNNER

FROM alpine

ARG profile=release

WORKDIR /usr/src
COPY --from=builder /usr/src/rustmark/target/x86_64-unknown-linux-musl/$profile/rustmark ./
COPY Config.toml ./

EXPOSE 8000

CMD ["/usr/src/rustmark"]



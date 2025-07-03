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
    case "$(uname -m)" in
        x86_64)  rustup target add x86_64-unknown-linux-musl ;;
        aarch64) rustup target add aarch64-unknown-linux-gnu ;;
        *)       echo "Unsupported architecture: $(uname -m)" >&2 ; exit 1 ;;
    esac
EOF

# Install build dependencies
WORKDIR /usr/src
RUN <<EOF
    set -e
    mkdir mold
    curl -SL https://github.com/rui314/mold/releases/download/v2.40.1/mold-2.40.1-$(uname -m)-linux.tar.gz \
        | tar --strip-components=1 -xzC mold
    mv mold/bin/mold /usr/local/bin/
    rm -rf mold
    mkdir upx
    case "$(uname -m)" in
        x86_64)  UPX_ARCH="amd64" ;;
        aarch64) UPX_ARCH="arm64" ;;
    esac
    curl -SL https://github.com/upx/upx/releases/download/v5.0.1/upx-5.0.1-${UPX_ARCH}_linux.tar.xz \
        | tar --strip-components=1 -xJC upx
    mv upx/upx /usr/local/bin/
    rm -rf upx
EOF

# Prepare build location
WORKDIR /usr/src
RUN <<EOF
    set -e
    mkdir -p rustmark/.cargo
    case "$(uname -m)" in
        x86_64)  RUST_TARGET="x86_64-unknown-linux-musl" ;;
        aarch64) RUST_TARGET="aarch64-unknown-linux-gnu" ;;
    esac
    printf '
[target.'$RUST_TARGET']
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/mold"]' \
        > rustmark/.cargo/config.toml
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
COPY Cargo.toml ./
RUN <<EOF
    set -e
    mkdir src
    echo "fn main() {}" > src/main.rs
    cp src/main.rs build.rs
    case "$(uname -m)" in
        x86_64)  RUST_TARGET="x86_64-unknown-linux-musl" ;;
        aarch64) RUST_TARGET="aarch64-unknown-linux-gnu" ;;
    esac
    cargo build --profile=$profile --target=$RUST_TARGET $cargo_opts
    rm build.rs
    rm src/main.rs
    rmdir src
    target_path=/usr/src/rustmark/target/$RUST_TARGET
    ln -s $target_path/debug $target_path/dev
EOF

# Copy files and build project
COPY build.rs ./
COPY src src
COPY html html
COPY static static
COPY content content
RUN <<EOF
    set -e
    touch src/main.rs
    touch build.rs
    case "$(uname -m)" in
        x86_64)  RUST_TARGET="x86_64-unknown-linux-musl" ;;
        aarch64) RUST_TARGET="aarch64-unknown-linux-gnu" ;;
    esac
    cargo build --profile=$profile --target=$RUST_TARGET $cargo_opts
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
        case "$(uname -m)" in
            x86_64)  RUST_TARGET="x86_64-unknown-linux-musl" ;;
            aarch64) RUST_TARGET="aarch64-unknown-linux-gnu" ;;
        esac
        upx --best target/$RUST_TARGET/$profile/rustmark
    fi
EOF


# RUNNER

# Platform-specific base images
FROM alpine:latest AS runner-amd64
FROM gcr.io/distroless/cc:latest AS runner-arm64

# Select the appropriate runner based on target architecture
FROM runner-${TARGETARCH}

ARG profile=release

WORKDIR /usr/src
COPY --from=builder /usr/src/rustmark/target/*/$profile/rustmark ./
COPY Config.docker.toml ./Config.toml

EXPOSE 8000

CMD ["/usr/src/rustmark"]



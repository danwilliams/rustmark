# syntax=docker/dockerfile:1.4

# BUILDER

FROM rust:1.70 as builder

# Prepare build location
WORKDIR /usr/src
RUN mkdir rustmark

# Initial non-project build to cache dependencies
WORKDIR /usr/src/rustmark
COPY Cargo.toml Cargo.lock ./
RUN <<EOF
    set -e
    mkdir src
    echo "fn main() {}" > src/main.rs
    cp src/main.rs build.rs
    cargo build --release
    rm build.rs
    rm src/main.rs
    rmdir src
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
    cargo build --release
    rm build.rs
    rmdir content
EOF

# Copy content files and build full project, including content
COPY build.rs ./
COPY content content
RUN <<EOF
    set -e
    touch build.rs
    cargo build --release
EOF


# RUNNER

FROM gcr.io/distroless/cc

WORKDIR /usr/src
COPY --from=builder /usr/src/rustmark/target/release/rustmark ./
COPY Config.toml ./

EXPOSE 8000

CMD ["/usr/src/rustmark"]



# BUILDER

FROM rust:1.70 as builder

# Prepare build location
WORKDIR /usr/src
RUN cargo new --bin rustmark

# Initial non-project build to cache dependencies
WORKDIR /usr/src/rustmark
COPY Cargo.toml Cargo.lock ./
RUN cp src/main.rs build.rs
RUN cargo build --release
RUN rm src/*.rs build.rs

# Copy source and content files
COPY build.rs Config.toml ./
COPY src src
COPY static static
COPY html html
COPY content content
RUN touch src/main.rs
RUN touch build.rs

# Build project
RUN rm target/release/deps/rustmark*
RUN cargo build --release


# RUNNER

FROM gcr.io/distroless/cc

WORKDIR /usr/src
COPY --from=builder /usr/src/rustmark/target/release/rustmark ./
COPY --from=builder /usr/src/rustmark/Config.toml ./

EXPOSE 8000

CMD ["/usr/src/rustmark"]



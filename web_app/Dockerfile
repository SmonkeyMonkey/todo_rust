ARG BASE_IMAGE=nwtgck/rust-musl-builder:1.71.0
# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `using-diesel`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/web_app \
    /usr/local/bin/

EXPOSE 8080

CMD ["/usr/local/bin/web_app"]
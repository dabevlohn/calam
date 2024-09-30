FROM rust:bookworm AS bin-builder

ARG CARGO_PROFILE=release

ENV RUST_BACKTRACE=1
ENV RUST_LOG=calam=info

RUN apt-get -y update \
    && apt-get -y install ca-certificates \
                          clang \
                          cmake \
                          libssl-dev \
                          llvm \
                          protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Required by tonic
RUN rustup component add rustfmt

COPY ./ /calam
WORKDIR /calam

RUN echo "Building workspace with and profile '$CARGO_PROFILE'" \
    && cargo build \
        -p calam \
        --bin calam \
        $(test "$CARGO_PROFILE" = "release" && echo "--release") \
    && echo "Copying binaries to /calam/bin" \
    && mkdir -p /calam/bin \
    && find target/$CARGO_PROFILE -maxdepth 1 -perm /a+x -type f -exec mv {} /calam/bin \;

FROM debian:bookworm-slim AS calam

LABEL org.opencontainers.image.title="Calam"
LABEL maintainer="V I R <vir@tue.su>"
LABEL org.opencontainers.image.vendor="Calam Foundation"
LABEL org.opencontainers.image.licenses="MIT"

RUN apt-get -y update \
    && apt-get -y install ca-certificates \
                          curl \
                          libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /calam
COPY --from=bin-builder /calam/bin/calam /usr/local/bin/calam

RUN calam --help

ENTRYPOINT ["calam"]

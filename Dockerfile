# Extract graphviz and dependencies
FROM rust:slim-bookworm AS deb_extractor
RUN cd /tmp && \
    apt-get update && apt-get download \
        graphviz libgvc6 libcgraph6 libltdl7 libxdot4 libcdt5 libpathplan4 libexpat1 zlib1g libgcc-s1 && \
    mkdir /dpkg && \
    for deb in *.deb; do dpkg --extract $deb /dpkg || exit 10; done

FROM rust:slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/base-debian11:latest AS run
COPY --from=deb_extractor /dpkg /
COPY --from=builder /app/target/release/graphview /graphview
COPY --from=builder /app/src/static /src/static
# Configure dot plugins
RUN ["dot", "-c"]

# Use a non-root user: slightly more secure (defense in depth)
USER nobody
WORKDIR /
ENTRYPOINT ["/graphview"]

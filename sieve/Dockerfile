FROM debian:buster-slim

# Install dependencies.
RUN apt-get -y update && \
    apt-get clean && \
    apt-get autoremove

# Prepares CA certificates to avoid SSL error described at
# https://www.reddit.com/r/rust/comments/an6z2t/cannot_verify_certificates_when_making_http.
#
# Solution found at
# https://tech.fpcomplete.com/blog/2018/07/deploying-rust-with-docker-and-kubernetes
RUN apt-get -y install ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Since when we build the bin, it resides in parent dir's "target" dir, we first
# need to move it from there to the docker context.
#
# This cannot be done from within the Dockerfile, that's why we use a helper
# script to build this image.
COPY sieve.bin /usr/local/bin/sieve

ENTRYPOINT ["sieve"]


FROM debian:buster-slim

# the gecko driver runs within the image as a background process
# see readme for the justification
ENV GECKO_URL "http://localhost:4444"

# see https://github.com/mozilla/geckodriver/releases
ENV GECKO_VERSION "0.29.0"

# Install dependencies.
RUN apt-get -y update && \
    apt-get -y install libpq-dev curl wget && \
    apt-get clean && \
    apt-get autoremove

# Prepares CA certificates to avoid SSL error described at
# https://www.reddit.com/r/rust/comments/an6z2t/cannot_verify_certificates_when_making_http.
#
# Solution found at
# https://tech.fpcomplete.com/blog/2018/07/deploying-rust-with-docker-and-kubernetes
RUN apt-get -y install ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Installing firefox and gecko driver for screenshots.
RUN apt-get -y update && apt-get install -y firefox-esr xvfb
RUN wget "https://github.com/mozilla/geckodriver/releases/download\
/v${GECKO_VERSION}/geckodriver-v${GECKO_VERSION}-linux64.tar.gz" && \
    tar -xvzf geckodriver* && \
    chmod +x geckodriver && \
    mv geckodriver /usr/local/bin/

COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod u+x "/usr/local/bin/entrypoint.sh"

# Since when we build the bin, it resides in parent dir's "target" dir, we first
# need to move it from there to the docker context.
#
# This cannot be done from within the Dockerfile, that's why we use a helper
# script to build this image.
COPY prtsc.bin /usr/local/bin/prtsc
RUN chmod u+x "/usr/local/bin/prtsc"

RUN firefox-esr --version

ENTRYPOINT ["entrypoint.sh"]


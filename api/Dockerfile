FROM debian:jessie
MAINTAINER Brandon Powers "bpowers1215@gmail.com"

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install \
       ca-certificates \
       curl \
       gcc \
       libc6-dev \
       gdb \
       g++-multilib \
       lib32stdc++6 \
       libssl-dev \
       libncurses5-dev \
       -qqy \
       --no-install-recommends

ENV RUST_ARCHIVE=rust-1.19.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir /rust
WORKDIR /rust

RUN curl -fsOSL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE \
    && ./install.sh

# install libsodium
RUN apt-get install -y pkg-config
RUN apt-get update && apt-get install -y libsodium-dev

# cleanup package manager
RUN apt-get remove --purge -y curl && apt-get autoclean && apt-get clean
RUN rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

VOLUME ["/var/lib/money_map/api", "/logs"]
WORKDIR /var/lib/money_map/api

EXPOSE 6767 27017

CMD ["cargo", "run"]

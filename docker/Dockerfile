FROM ubuntu:latest

RUN \
    triple=x86_64-unknown-linux-gnu \
    && apt-get update \
    && apt-get install -y \
        curl \
        gdb \
        g++-multilib \
        lib32stdc++6 \
        libssl-dev \
        libncurses5-dev \
        sudo \
        runit \
    && curl -sL https://static.rust-lang.org/dist/rust-nightly-$triple.tar.gz | tar xvz -C /tmp \
    && /tmp/rust-nightly-$triple/install.sh \
    && curl -sL https://static.rust-lang.org/cargo-dist/cargo-nightly-$triple.tar.gz | tar xvz -C /tmp \
    && /tmp/cargo-nightly-$triple/install.sh \
    && apt-get remove --purge -y curl && apt-get autoclean && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY entrypoint.sh /
ENTRYPOINT ["/entrypoint.sh"]

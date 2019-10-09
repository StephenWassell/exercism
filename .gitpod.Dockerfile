FROM gitpod/workspace-full

USER root

# Install custom tools, runtime, etc. using apt-get
# For example, the command below would install "bastet" - a command line tetris clone:
#
# RUN apt-get update \
#    && apt-get install -y bastet \
#    && apt-get clean && rm -rf /var/cache/apt/* && rm -rf /var/lib/apt/lists/* && rm -rf /tmp/*
#
# More information: https://www.gitpod.io/docs/42_config_docker/

RUN rustup component add rustfmt --toolchain stable-x86_64-unknown-linux-gnu \
    && rustup component add clippy --toolchain stable-x86_64-unknown-linux-gnu \
    && npm install -g elm \
    && npm install -g elm-test \
    && npm install -g elm-format

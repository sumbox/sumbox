ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT} AS development

ARG MOLD_VERSION=1.4.2

COPY ps1.bash .
RUN cat ps1.bash >> ~/.bashrc && sudo rm ps1.bash

COPY .bash_aliases /home/vscode/.bash_aliases

# Enable our git hooks and set the permisisons on docker sock.
RUN echo 'git config core.hooksPath /vscode/.devcontainer/.githooks' >> ~/.bashrc

# all the volumes configured in the docker-compose.yml
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN sudo mkdir -p /workspace/target && sudo chown $USERNAME:$USERNAME /workspace/target

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang \ 
        lld \
        bash \
        libpq-dev \
        build-essential \
        ca-certificates \
        gnupg \
        # For musl builds
        musl-dev \
        musl-tools \
        musl \
   && apt-get autoremove -y && apt-get clean -y



# Install PSQL 14 client 
RUN sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list' \
   && wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - \
   && apt-get -y update \
   && apt-get -y install postgresql-client \
   && apt-get autoremove -y && apt-get clean -y


# Install node.
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash -
RUN sudo apt-get install -y nodejs

USER $USERNAME

RUN cargo install cargo-chef --locked

RUN curl https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-unknown-linux-musl.tar.xz -L -o cargo-watch.tar.xz \
    && tar -xf cargo-watch.tar.xz \
    && mv cargo-watch-v7.8.0-x86_64-unknown-linux-musl/cargo-watch $CARGO_HOME/bin

# Essential tools
RUN rustup component add rustfmt clippy

# For musl builds
RUN rustup target add x86_64-unknown-linux-musl


# Pulumi for infrastructure as code.
RUN curl -fsSL https://get.pulumi.com | sudo -E bash -
RUN sudo chown -R $USERNAME:$USERNAME /home/$USERNAME/.pulumi

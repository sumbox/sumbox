ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT} AS development

ARG MOLD_VERSION=1.6.0

COPY ps1.bash .
RUN set -eux; \
    cat ps1.bash >> ~/.bashrc && sudo rm ps1.bash

COPY .bash_aliases /home/vscode/.bash_aliases

# Enable our git hooks and set the permisisons on docker sock.
RUN echo 'git config core.hooksPath /vscode/.devcontainer/.githooks' >> ~/.bashrc

# all the volumes configured in the docker-compose.yml
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN set -eux; \
    sudo mkdir -p /workspace/target && sudo chown $USERNAME:$USERNAME /workspace/target

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN set -eux; \
    apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang \ 
        lld \
        bash \
        libpq-dev \
        build-essential \
        ca-certificates \
        gnupg \
   && apt-get autoremove -y && apt-get clean -y


# Install PSQL 14 client 
RUN set -eux; \
   sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list' \
   && wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - \
   && apt-get -y update \
   && apt-get -y install postgresql-client \
   && apt-get autoremove -y && apt-get clean -y


# Install node.
RUN set -eux; \
    curl -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash -
RUN set -eux; \
    sudo apt-get install -y nodejs

RUN set -eux; \
    curl https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-unknown-linux-musl.tar.xz -L -o cargo-watch.tar.xz \
    && tar -xf cargo-watch.tar.xz \
    && mv cargo-watch-v7.8.0-x86_64-unknown-linux-musl/cargo-watch $CARGO_HOME/bin

RUN set -eux; \
    curl --fail --location "https://github.com/rui314/mold/releases/download/v${MOLD_VERSION}/mold-${MOLD_VERSION}-x86_64-linux.tar.gz" --output /tmp/mold.tar.gz  \
    && tar --directory "/usr/local" -xzvf "/tmp/mold.tar.gz" --strip-components 1 \ 
    && rm /tmp/mold.tar.gz

USER $USERNAME

# Install cargo-chef
RUN set -eux; \
    cargo install cargo-chef --locked

# Install sscache
RUN set -eux; \
    cargo install sccache --locked

RUN set -eux; \
    cargo install cargo-make --locked 
    
# Essential tools
RUN set -eux; \
    rustup component add rustfmt clippy 

# Pulumi for infrastructure as code.
RUN set -eux; \
    curl -fsSL https://get.pulumi.com | sudo -E bash -
RUN set -eux; \
    sudo chown -R $USERNAME:$USERNAME /home/$USERNAME/.pulumi
RUN set -eux; \
    SCCACHE=`which sccache` && mkdir $HOME/.cargo && \
    echo "[build]" > $HOME/.cargo/config.toml && \
    echo "rustc-wrapper = "$SCCACHE"" >> $HOME/.cargo/config.toml
RUN set -eux; \
    git config --global --add safe.directory /workspace
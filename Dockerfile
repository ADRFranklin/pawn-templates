FROM rust:1.37

WORKDIR /root
COPY . .
RUN apt update && apt install -y gcc-multilib && curl -sL https://taskfile.dev/install.sh | sh
RUN bin/task toolchain-linux

ENTRYPOINT [ "make", "build-linux-release" ]

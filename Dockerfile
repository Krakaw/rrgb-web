FROM mitchallen/pi-cross-compile

SHELL ["/bin/bash", "-c"]

RUN apt update && apt install -y libclang-dev curl llvm && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -t arm-unknown-linux-gnueabihf

COPY . .
RUN source $HOME/.cargo/env && \
     cargo build --target=arm-unknown-linux-gnueabihf

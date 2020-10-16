FROM tttapa/rpi-cross-toolchain:armv6-rpi-linux-gnueabihf

#SHELL ["/bin/bash", "-c"]
#apt update && apt install -y libclang-dev curl llvm && \
USER root
RUN yum -y install clang
USER develop
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -t arm-unknown-linux-gnueabihf && \
    source $HOME/.cargo/env
#    export PATH="/pitools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin:$PATH" && \
ENV PATH=/home/develop/.cargo/bin:$PATH
# mkdir /root/ct-ng
# apt install -y dh-autoreconf flex
# apt-get install gperf bison flex texinfo unzip help2man gawk
# sudo apt install libtool-bin libtool-doc libncurses-dev
# git clone https://github.com/crosstool-ng/crosstool-ng
# cd crosstool-ng
# ./bootstrap
# ./configure --prefix
# make
# make install
# export PATH="$PATH:/root/ct-ng/bin"
#RUN ./scripts/build.sh --target=arm-unknown-linux-gnueabihf

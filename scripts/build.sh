#!/usr/bin/env bash

export RPI_WS281X_SYSROOT=/home/develop/x-tools/armv6-rpi-linux-gnueabihf/armv6-rpi-linux-gnueabihf/sysroot
export CC_arm_unknown_linux_gnueabihf=armv6-rpi-linux-gnueabihf-gcc
export AR_arm_unknown_linux_gnueabihf=armv6-rpi-linux-gnueabihf-gcc

cargo build $@

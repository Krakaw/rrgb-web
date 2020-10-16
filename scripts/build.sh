#!/usr/bin/env bash

export RPI_WS281X_SYSROOT=/pitools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/arm-linux-gnueabihf/sysroot
export CC_arm_unknown_linux_gnueabihf=/pitools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-gcc
export AR_arm_unknown_linux_gnueabihf=/pitools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-ar

cargo build $@

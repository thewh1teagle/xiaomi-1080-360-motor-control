#!/bin/bash

arm-linux-gnueabihf-gcc $1 -L$(pwd)/lib \
	-l:device_kit.so -l:libmi_isp.so -l:boardav.so -l:libmi.so -l:libmiio_util.so.0.0.1 \
	-l:libpthread-2.25.so -l:libjson-c.so -l:libMTE_LINUX.so -l:libmortoxev.so.0.0.0 \
	-l:libmortox_share.so.0.0.0 -l:libmortox.so.0.0.0  -l:libOMX_AVQE_A.so -l:libOMX_BELA.so \
	-l:libdev.so.4 -l:librt-2.25.so -l:libc-2.28.so \
	-o $2

# -l:libc-2.25.so
#!/bin/bash

set -e

i686-linux-gnu-gcc \
    -Wall -Werror \
    -O0 -g -fno-strict-aliasing -fno-pie -fno-stack-protector \
    -mno-sse -mno-avx \
    -nostdlib -ffreestanding -static -m32 \
    -o test-i386 \
    test-i386.c the_libc.c printf.c \
    -lgcc \
    

i686-linux-gnu-gcc \
    -Wall -Werror \
    -O0 -g -fno-strict-aliasing -fno-pie -fno-stack-protector \
    -mno-sse -mno-avx \
    -nostdlib -ffreestanding -static -m32 \
    -DTARGET_LINUX \
    -o test-i386-linux \
    test-i386.c the_libc.c printf.c \
    -lgcc \

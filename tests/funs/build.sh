#!/bin/bash
set -ex

# TODO:
nasm -felf32 -w+ptr msvc_float.asm
ld -melf_i386 msvc_float.o -o msvc_float

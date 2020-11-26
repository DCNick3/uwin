#!/bin/bash

cd "$(dirname "$0")"

rm -rf output
mkdir -p output

kaitai-struct-compiler --cpp-standard 11 --cpp-namespace uwin::win32::ldr --target cpp_stl --outdir output/ *.ksy

python3 patcher.py output/*

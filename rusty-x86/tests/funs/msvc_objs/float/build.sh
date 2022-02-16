#!/bin/bash
set -ex

nasm -fwin32 msvc_float.asm -o msvc_float.obj
../vc98.sh link msvc_float.obj /NODEFAULTLIB /ENTRY:start /SUBSYSTEM:NATIVE,117.119 /OUT:main.exe /MAP:main.map
rm ./*.obj

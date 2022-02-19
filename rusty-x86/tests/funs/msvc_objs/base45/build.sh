#!/bin/bash
set -ex
../vc98.sh cl base45.c /Ox /c
../vc98.sh link base45.obj /NODEFAULTLIB /ENTRY:start /SUBSYSTEM:NATIVE,117.119 /OUT:main.exe /MAP:main.map
rm ./*.obj
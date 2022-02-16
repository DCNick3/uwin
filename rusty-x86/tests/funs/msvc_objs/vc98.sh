#!/bin/bash
VC98="/home/dcnick3/trash/homm3-switch/media/ms-sdk/Visual Studio 6.0/VC98/"

export INCLUDE="$VC98/include"
export LIB="$VC98/lib"

TOOL="$1"
shift

wine "$VC98/bin/$TOOL.exe" "$@"


#!/bin/bash

set -e

# NOTICE: the fixup for magic_MessageBoxA is no longer needed (I changed it)
# preserved just in case

clang -c -emit-llvm main.c -o main.bc
llvm-link main.bc test.ll -o test_comb.bc && llvm-dis test_comb.bc
# fixup the test_comb.bc (we generate calls with fastcc calling convention and I have no idea how to make clang emit these)
sed -i 's/call void @sub_00401000/call fastcc void @sub_00401000/;s/define dso_local void @magic_MessageBoxA/define dso_local fastcc void @magic_MessageBoxA/' test_comb.ll
clang test_comb.ll -o test_comb -g3

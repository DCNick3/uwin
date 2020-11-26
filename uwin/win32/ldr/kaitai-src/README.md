Even though compiling .ksy files is not part of the build process (and must be done manually), the source files are included in this directory.

It was not made so because:

1) kaitai workflow is much different from c++ workflow. You just dont go around editing it and using right away. You usually go with kaitai IDE to generate the structure you need and incorporate it in your code once
2) incorporating it would add java as build dependency, which is undesirable

Running `./build.sh` will build and patch the kaitai stuct code, placing it into `output/` directory
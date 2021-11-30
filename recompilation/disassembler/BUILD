load("@pip_parsed_deps//:requirements.bzl", "requirement")

py_binary(
  name = "disassembler",
  srcs = ["disassembler.py"],
  deps = [
    requirement("iced-x86"),
    requirement("makeelf"),
    requirement("bitarray"),
  ],
)

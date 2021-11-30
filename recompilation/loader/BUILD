load("@pip_parsed_deps//:requirements.bzl", "requirement")

py_binary(
  name = "loader",
  srcs = ["loader.py"],
  deps = [
    requirement("pefile"),
    "//vendor/makeelf",
  ],
)


load("@rules_python//python:pip.bzl", "pip_parse")


def repo():
  pip_parse(
      name = "pip_parsed_deps",
      # (Optional) You can provide a python_interpreter (path) or a python_interpreter_target (a Bazel target, that
      # acts as an executable). The latter can be anything that could be used as Python interpreter. E.g.:
      # 1. Python interpreter that you compile in the build file (as above in @python_interpreter).
      # 2. Pre-compiled python interpreter included with http_archive
      # 3. Wrapper script, like in the autodetecting python toolchain.
      python_interpreter_target = "@python_interpreter//:interpreter_link",

      requirements_lock = "//third_party/python_libraries:requirements_lock.txt",
  )

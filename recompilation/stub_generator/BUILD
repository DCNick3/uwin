load("@pip_parsed_deps//:requirements.bzl", "requirement")

py_binary(
  name = "stub_generator",
  srcs = ["stub_generator.py", "def_parser.py"],
  deps = [
    requirement("pefile"),
    requirement("pyparsing"),
  ],
)

py_test(
  name = "def_parser_test",
  srcs = ["def_parser.py", "def_parser_test.py"],
  deps = [
    requirement("pyparsing"),
    requirement("pytest"),
  ]
)

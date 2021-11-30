py_library(
  name = "makeelf",
  deps = [],
  srcs = glob(["makeelf/**/*.py"]),
  imports = [":."],
  visibility = ["//visibility:public"],
)

py_test(
  name = "test_elf",
  deps = [":makeelf"],
  srcs = ["makeelf/test_elf.py"],
)

py_test(
  name = "test_elfsect",
  deps = [":makeelf"],
  srcs = ["makeelf/test_elfsect.py"],
)

py_test(
  name = "test_elfstruct",
  deps = [":makeelf"],
  srcs = ["makeelf/test_elfstruct.py"],
)

load("@remill//build_tools/BCCompiler:bc-compiler.bzl", "bc_object")

load("//build_tools/embed_data:build_defs.bzl", "cc_embed_data")

bc_object(
  name = "uwin_intrinsics_bc",
  src = "intrinsics.cpp",
  additional_deps = [
    "@remill//include:all_headers"
  ],
  definitions = dict(),
  include_directories = [
    "@remill//include:."
  ],
  target = "x86_64",
  opts = [
    "-O3",
    "-g0",
    "-m32",
  ],
)

cc_embed_data(
  name = "uwin_intrinsics",
  srcs = [
    ":uwin_intrinsics_bc"
  ],
  cpp_namespace = "remill",
  h_file_output = "uwin_intrinsics.h",
)

cc_binary(
  name = "lifter",
  deps = [
    ":uwin_intrinsics",
    "@com_github_serge1_elfio//:elfio",
    "@remill//:remill",
    "@llvm-project//llvm:Core",
    "@com_github_nlohmann_json//:json",
    "//vendor/range_map",
  ],
  srcs = glob(["src/**/*.cpp", "src/**/*.h"]),
)


load("@uwin//third_party/cpp/xed:xed_rule.bzl", "compile_xed")

exports_files([ "." ])

filegroup(
    name = "all_sources",
    srcs = glob(["**/*"])
)


py_binary(
    name = "mfile",
    srcs = [
        "mfile.py",
        "xed_build_common.py",
        "xed_mbuild.py"
    ],
    main = "mfile.py",
    deps = ["@com_github_intelxed_mbuild//:mbuild"]
)

compile_xed(
    name = "xed",
    mfile = ":mfile",
    all_sources = ":all_sources",
    source_root = ":.",
    visibility = ["//visibility:public"]
)

"""Provides the repository macro to import LLVM."""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def repo():
    """Imports LLVM."""
    XED_COMMIT = "5976632eeaaaad7890c2109d0cfaf4012eaca3b8"
    XED_SHA256 = "8cc7c3ee378a8827e760e71ecf6642a814f0a991d768a120df5f177323b692e6"
    
    MBUILD_COMMIT = "3e8eb33aada4153c21c4261b35e5f51f6e2019e8"
    MBUILD_SHA256 = "bbbff8cccbb56d9614f6fa163f1546a1619fe331030c4f92db6387e63a88ba41"

    http_archive(
        name = "com_github_intelxed_mbuild",
        sha256 = MBUILD_SHA256,
        strip_prefix = "mbuild-{commit}".format(commit = MBUILD_COMMIT),
        urls = [
            "https://github.com/intelxed/mbuild/archive/{commit}.tar.gz".format(commit = MBUILD_COMMIT),
        ],
        build_file = "//third_party/xed:mbuild.BUILD",
        patches = [
            "//third_party/xed:mbuild_abs_compiler_path.patch"
        ]
    )

    http_archive(
        name = "com_github_intelxed_xed",
        sha256 = XED_SHA256,
        strip_prefix = "xed-{commit}".format(commit = XED_COMMIT),
        urls = [
            "https://github.com/intelxed/xed/archive/{commit}.tar.gz".format(commit = XED_COMMIT),
        ],
        build_file = "//third_party/xed:xed.BUILD",
    )

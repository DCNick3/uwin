"""Provides the repository macro to import LLVM."""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def repo():
    """Imports LLVM."""
    LLVM_COMMIT = "4b4bc1ea16dec58e828ec3a0154046b10ec69242"
    LLVM_SHA256 = "a744dd2d7241daf6573ef88f9bd61b9d690cbac73d1f26381054d74e1f3505e0"

    http_archive(
        name = "llvm-raw",
        sha256 = LLVM_SHA256,
        strip_prefix = "llvm-project-{commit}".format(commit = LLVM_COMMIT),
        urls = [
            "https://github.com/llvm/llvm-project/archive/{commit}.tar.gz".format(commit = LLVM_COMMIT),
        ],
        build_file = "//third_party/cpp/llvm:llvm.BUILD",
    )

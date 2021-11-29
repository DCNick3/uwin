"""Provides the repository macro to import LLVM."""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def repo():
    ELFIO_VERSION = "3.9"
    ELFIO_SHA256 = "767b269063fc35aba6d361139f830aa91c45dc6b77942f082666876c1aa0be0f"

    http_archive(
        name = "com_github_serge1_elfio",
        sha256 = ELFIO_SHA256,
        strip_prefix = "elfio-{version}".format(version = ELFIO_VERSION),
        urls = [
            "https://github.com/serge1/ELFIO/releases/download/Release_{version}/elfio-{version}.tar.gz".format(version = ELFIO_VERSION),
        ],
        build_file = "//third_party/cpp/elfio:elfio.BUILD",
    )

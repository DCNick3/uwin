load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("//third_party/sysroots/debian:debian_sysroot.bzl", "debian_sysroot")

def repo():
    debian_sysroot(
        name = "bitcode_sysroot_amd64",
        arch = "amd64",
        distro = "stretch",
        packages = [
            # no dependencies are resolved here (because they don't make sense for us anyways)
            # only the bare minimum should be specified here
            "libc6-dev",
            "libc6-dev-i386",
            "libstdc++-6-dev",
            "linux-libc-dev",
        ]
    )

    debian_sysroot(
        name = "bitcode_sysroot_arm64",
        arch = "arm64",
        distro = "stretch",
        packages = [
            # no dependencies are resolved here (because they don't make sense for us anyways)
            # only the bare minimum should be specified here
            "libc6-dev",
            "libstdc++-6-dev",
            "linux-libc-dev",
        ]
    )

    debian_sysroot(
        name = "bitcode_sysroot_armhf",
        arch = "armhf",
        distro = "stretch",
        packages = [
            # no dependencies are resolved here (because they don't make sense for us anyways)
            # only the bare minimum should be specified here
            "libc6-dev",
            "libstdc++-6-dev",
            "linux-libc-dev",
        ]
    )


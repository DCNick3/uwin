load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

def repo():
  SKYLIB_VERSION = "1.0.3"
  STARTDOC_COMMIT = "8f6d22452d088b49b13ba2c224af69ccc8ccbc90"
  RULES_PYTHON_VERSION = "0.5.0"

  http_archive(
      name = "bazel_skylib",
      sha256 = "1c531376ac7e5a180e0237938a2536de0c54d93f5c278634818e0efc952dd56c",
      urls = [
          "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/{version}/bazel-skylib-{version}.tar.gz".format(version=SKYLIB_VERSION),
          "https://github.com/bazelbuild/bazel-skylib/releases/download/{version}/bazel-skylib-{version}.tar.gz".format(version=SKYLIB_VERSION),
      ],
  )

  git_repository(
      name = "io_bazel_stardoc",
      commit = STARTDOC_COMMIT,
      remote = "https://github.com/bazelbuild/stardoc.git",
      shallow_since = "1620849756 -0400"
  )

  http_archive(
      name = "rules_python",
      url = "https://github.com/bazelbuild/rules_python/releases/download/{version}/rules_python-{version}.tar.gz".format(version=RULES_PYTHON_VERSION),
      sha256 = "cd6730ed53a002c56ce4e2f396ba3b3be262fd7cb68339f0377a45e8227fe332",
  )
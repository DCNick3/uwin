load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def repo():
  GFLAGS_VERSION = "2.2.2"
  GLOG_VERSION = "0.5.0"
  GTEST_VERSION = "1.11.0"

  http_archive(
    name = "com_github_gflags_gflags",
    sha256 = "34af2f15cf7367513b352bdcd2493ab14ce43692d2dcd9dfc499492966c64dcf",
    strip_prefix = "gflags-{version}".format(version=GFLAGS_VERSION),
    urls = ["https://github.com/gflags/gflags/archive/v{version}.tar.gz".format(version=GFLAGS_VERSION)],
  )

  http_archive(
    name = "com_github_google_glog",
    sha256 = "21bc744fb7f2fa701ee8db339ded7dce4f975d0d55837a97be7d46e8382dea5a",
    strip_prefix = "glog-{version}".format(version=GLOG_VERSION),
    urls = ["https://github.com/google/glog/archive/v{version}.zip".format(version=GLOG_VERSION)],
  )

  http_archive(
    name = "com_google_googletest",
    strip_prefix = "googletest-release-{version}".format(version=GTEST_VERSION),
    sha256 = "b4870bf121ff7795ba20d20bcdd8627b8e088f2d1dab299a031c1034eddc93d5",
    urls = ["https://github.com/google/googletest/archive/refs/tags/release-{version}.tar.gz".format(version=GTEST_VERSION)],
  )
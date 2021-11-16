load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")


def repo():
  TOOLCHAIN_COMMIT = "072cf8358ee3d129ee44ecd38cdd878c2a7944b5"

  # native.local_repository(
  #   name = "rules_cc_toolchain",
  #   path = "/home/dcnick3/git_cloned/bazel_rules_cc_toolchain",
  # )


  git_repository(
      name = "rules_cc_toolchain",
      commit = TOOLCHAIN_COMMIT,
      remote = "https://github.com/DCNick3/bazel_rules_cc_toolchain",
      shallow_since = "1637096381 +0300",
  )
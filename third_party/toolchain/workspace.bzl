load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")


def repo():
  TOOLCHAIN_COMMIT = "02e19d5a3559370982cc8849b151a824079c2eec"

  # native.local_repository(
  #   name = "rules_cc_toolchain",
  #   path = "/home/dcnick3/git_cloned/bazel_rules_cc_toolchain",
  # )


  git_repository(
      name = "rules_cc_toolchain",
      commit = TOOLCHAIN_COMMIT,
      remote = "https://github.com/DCNick3/bazel_rules_cc_toolchain",
      shallow_since = "1638387355 +0300",
  )
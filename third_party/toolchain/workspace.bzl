load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")


def repo():
  TOOLCHAIN_COMMIT = "808d43016de6795a94635257d948d21280399fae"

  # native.local_repository(
  #   name = "rules_cc_toolchain",
  #   path = "/home/dcnick3/git_cloned/bazel_rules_cc_toolchain",
  # )


  git_repository(
      name = "rules_cc_toolchain",
      commit = TOOLCHAIN_COMMIT,
      remote = "https://github.com/DCNick3/bazel_rules_cc_toolchain",
      shallow_since = "1638453785 +0300",
  )
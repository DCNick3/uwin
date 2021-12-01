load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

def repo():
  REMILL_COMMIT = "1434c3e102b781690c764fb8a21cdba3380a8b06"

  # native.local_repository(
  #   name = "remill",
  #   path = "/home/dcnick3/trash/homm3-switch/code/lifting/remill",
  # )

  git_repository(
      name = "remill",
      commit = REMILL_COMMIT,
      remote = "https://github.com/DCNick3/uwin-remill.git",
      shallow_since = "1638394651 +0300",
  )
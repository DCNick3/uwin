load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

def repo():
  REMILL_COMMIT = "5ed5108d618a432a5d6b757d8a7824876f4ab11b"
  
  git_repository(
      name = "remill",
      commit = REMILL_COMMIT,
      remote = "https://github.com/DCNick3/uwin-remill.git",
      shallow_since = "1637243998 +0300",
  )
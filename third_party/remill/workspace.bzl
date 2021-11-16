load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

def repo():
  REMILL_COMMIT = "9c3aba1474de25ac7792d6512b93ef3a282dc002"
  
  git_repository(
      name = "remill",
      commit = REMILL_COMMIT,
      remote = "https://github.com/DCNick3/uwin-remill.git",
      shallow_since = "1637091226 +0300",
  )
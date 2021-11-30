load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

def repo():
  REMILL_COMMIT = "0c5a22f9fb79b0a491092a3c7759d75aa340e2e4"

  git_repository(
      name = "remill",
      commit = REMILL_COMMIT,
      remote = "https://github.com/DCNick3/uwin-remill.git",
      shallow_since = "1638269500 +0300",
  )
workspace(
    name = "uwin"
)

# =====
# setup the toolchain and common bazel libs
# =====
load("//third_party/common_bazel_libs:workspace.bzl", common_bazel_libs_repo = "repo")
common_bazel_libs_repo()
load("//third_party/common_bazel_libs:setup.bzl", common_bazel_libs_setup = "setup")
common_bazel_libs_setup()

load("//third_party/toolchain:workspace.bzl", toolchain_repo = "repo")
toolchain_repo()
load("//third_party/toolchain:setup.bzl", toolchain_setup = "setup")
toolchain_setup()

# =====
# python
# =====
load("//third_party/python:workspace.bzl", python_repo = "repo")
python_repo()
load("//third_party/python:setup.bzl", python_setup = "setup")
python_setup()

# =====
# python libraries
# =====
load("//third_party/python_libraries:workspace.bzl", python_libraries_repo = "repo")
python_libraries_repo()
load("//third_party/python_libraries:setup.bzl", python_libraries_setup = "setup")
python_libraries_setup()

# =====
# gtest, glog, gflags
# =====
load("//third_party/google_libs:workspace.bzl", google_libs_repo = "repo")
google_libs_repo()


# =====
# llvm
# =====
load("//third_party/llvm:workspace.bzl", llvm_repo = "repo")
llvm_repo()
load("//third_party/llvm:setup.bzl", llvm_setup = "setup")
llvm_setup()

# =====
# intel xed
# =====
load("//third_party/xed:workspace.bzl", xed_repo = "repo")
xed_repo()

# =====
# sysroots (as of now - only used by remill to build bitcode)
# =====
load("//third_party/sysroots:workspace.bzl", sysroots_repo = "repo")
sysroots_repo()

# =====
# remill
# =====
load("//third_party/remill:workspace.bzl", remill_repo = "repo")
remill_repo()

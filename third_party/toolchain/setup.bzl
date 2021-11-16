load("@rules_cc_toolchain//:rules_cc_toolchain_deps.bzl", "rules_cc_toolchain_deps")
load("@rules_cc_toolchain//config:rules_cc_toolchain_config_repository.bzl", "rules_cc_toolchain_config")
load("@rules_cc_toolchain//cc_toolchain:cc_toolchain.bzl", "register_cc_toolchains")

def setup():  
  rules_cc_toolchain_deps()
  rules_cc_toolchain_config(name = "rules_cc_toolchain_config")
  register_cc_toolchains()
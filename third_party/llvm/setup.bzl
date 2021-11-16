load("@llvm-raw//utils/bazel:configure.bzl", "llvm_configure", "llvm_disable_optional_support_deps")

def setup():
    llvm_disable_optional_support_deps()

    # Build @llvm-project from @llvm-raw using overlays.
    llvm_configure(
        name = "llvm-project",
        targets = [ 
            # no need for other llvm targets yet...
            "AArch64",
            "X86"
        ]
    ) 

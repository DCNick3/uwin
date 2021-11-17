
# Based on https://thundergolfer.com/bazel/python/2021/06/25/a-basic-python-bazel-toolchain/

KNOWN_PYTHON_DISTROS = {
  "x86_64-unknown-linux-gnu": {
    "url": "https://github.com/indygreg/python-build-standalone/releases/download/20211017/cpython-3.10.0-x86_64-unknown-linux-gnu-pgo+lto-20211017T1616.tar.zst",
    "sha256": "ba898cb849b02e9f6f567a3151435d5f6ba4dd3962890a2e45503677c6bd203c",
  },
  #"x86_64-apple-darwin": {
  #  "url": "https://github.com/indygreg/python-build-standalone/releases/download/20210228/cpython-3.8.8-x86_64-apple-darwin-pgo+lto-20210228T1503.tar.zst",
  #  "sha256": "4c859311dfd677e4a67a2c590ff39040e76b97b8be43ef236e3c924bff4c67d2",
  #},
}

def _detect_target_triple(repository_ctx):
  os_name = repository_ctx.os.name.lower()
  if os_name == "linux":
    uname = repository_ctx.execute(["uname", "-m"])
    return uname.stdout.strip() + "-unknown-linux-gnu"
  fail("Unknown OS:", os_name)

def _python_build_standalone_interpreter_impl(repository_ctx):

    target_triple = _detect_target_triple(repository_ctx)

    py_distro = KNOWN_PYTHON_DISTROS[target_triple]

    # TODO(Jonathon): Just use download_and_extract when it supports zstd. https://github.com/bazelbuild/bazel/pull/11968
    repository_ctx.download(
        url = [py_distro["url"]],
        sha256 = py_distro["sha256"],
        output = "python.tar.zst",
    )

    # TODO(Jonathon): NOT HERMETIC. Need to install 'unzstd' in rule and use it.
    unzstd_bin_path = repository_ctx.which("unzstd")
    if unzstd_bin_path == None:
        fail("On OSX and Linux this Python toolchain requires that the zstd and unzstd exes are available on the $PATH, but it was not found.")

    # NOTE: *Not Hermetic*. Need to install 'unzstd' in rule and use it.
    res = repository_ctx.execute([unzstd_bin_path, "python.tar.zst"])

    if res.return_code:
        fail("Error decompressiong with zstd" + res.stdout + res.stderr)

    repository_ctx.extract(archive = "python.tar")
    repository_ctx.delete("python.tar")
    repository_ctx.delete("python.tar.zst")

    # NOTE: 'json' library is only available in Bazel 4.*.
    python_build_data = json.decode(repository_ctx.read("python/PYTHON.json"))

    repository_ctx.symlink("python/" + python_build_data["python_exe"], "interpreter_link")

    BUILD_FILE_CONTENT = """
exports_files(["interpreter_link"])
filegroup(
    name = "files",
    srcs = glob(["install/**"], exclude = ["**/* *"]),
    visibility = ["//visibility:public"],
)
filegroup(
    name = "interpreter",
    srcs = ["python/{interpreter_path}"],
    visibility = ["//visibility:public"],
)
""".format(interpreter_path = python_build_data["python_exe"])

    repository_ctx.file("BUILD.bazel", BUILD_FILE_CONTENT)
    return None


_python_build_standalone_interpreter = repository_rule(
    implementation = _python_build_standalone_interpreter_impl,
    attrs = {
      "zstd": attr.label(
        default = "//third_party/zstd:zstdcli-frugal",
        allow_single_file = True,
      )
    },
)

def repo():
  _python_build_standalone_interpreter(
    name = "python_interpreter",
  )

load("@bazel_skylib//lib:paths.bzl", "paths")

#def _extract_deb_urls():

def _make_index_filename(distro, channel, arch):
  return "%s-%s-%s-Packages.xz" % (distro, channel, arch)

def _parse_hashes(hashes):
  res = dict()
  for line in hashes.split('\n'):
    if not line:
      continue
    name, hash = line.split(' ')
    res[name] = hash
  return res


def _check_python_output(repository_ctx, cmd, timeout = 60):
  python_bin = repository_ctx.which("python3")
  if not python_bin:
      # Windows typically just defines "python" as python3. The script itself
      # contains a check to ensure python3.
      python_bin = repository_ctx.which("python")

  if not python_bin:
      fail("Failed to find python3 binary")
  
  exec_result = repository_ctx.execute([python_bin] + cmd, timeout = timeout)

  if exec_result.return_code != 0:
      fail(("Failed to execute script: '{cmd}'\n" +
            "Exited with code {return_code}\n" +
            "stdout:\n{stdout}\n" +
            "stderr:\n{stderr}\n").format(
          cmd = " ".join([str(arg) for arg in cmd]),
          return_code = exec_result.return_code,
          stdout = exec_result.stdout,
          stderr = exec_result.stderr,
      ))
  
  return exec_result.stdout

def _impl(repository_ctx):

  # 0. get some paths
  sysroots_debian_path = repository_ctx.path(Label("//third_party/sysroots/debian:debian_sysroot.bzl")).dirname
  extract_deb_urls_path = sysroots_debian_path.get_child("extract_deb_urls.py")
  make_sysroot_path = sysroots_debian_path.get_child("make_sysroot.py")

  # 1. download the hashes of the indices & parse them
  repository_ctx.download(
    url = repository_ctx.attr.indices_release_url + '/hashes.txt',
    sha256 = repository_ctx.attr.indices_hashes_sha256,
    output = 'hashes.txt'
  )

  hashes = _parse_hashes(repository_ctx.read('hashes.txt'))
  repository_ctx.delete('hashes.txt')

  # 2. download the required indices
  required_indices = []
  for chan in repository_ctx.attr.channels:
    required_indices.append(_make_index_filename(
      repository_ctx.attr.distro,
      chan,
      repository_ctx.attr.arch,
    ))
    required_indices.append(_make_index_filename(
      repository_ctx.attr.distro,
      chan,
      "all",
    ))
  
  for index in required_indices:
    repository_ctx.download(
      url = paths.join(repository_ctx.attr.indices_release_url, index),
      output = index,
      sha256 = hashes[index],
    )

  # 3. extract the deb urls from the indices

  urls = json.decode(_check_python_output(repository_ctx,
    [extract_deb_urls_path, '-i'] + required_indices + ['-p'] + repository_ctx.attr.packages))

  for index in required_indices:
    repository_ctx.delete(index)

  # 4. download debs using urls we got
  debs = []
  for url in urls:
    base = paths.basename(url['filename'])
    repository_ctx.download(
      url = paths.join(repository_ctx.attr.debian_mirror_url, url['filename']),
      output = base,
      sha256 = url["sha256"],
    )
    debs.append(base)

  # 5. build the sysroot out of the downloaded debs
  _check_python_output(repository_ctx, [ make_sysroot_path, '-o', '.' ] + debs)

  for deb in debs:
    repository_ctx.delete(deb)

  repository_ctx.file("BUILD", """

exports_files([ "." ])

filegroup(
    name = "all_files",
    srcs = glob(["**/*"]),
    visibility = ["//visibility:public"],
) 
""")

debian_sysroot = repository_rule(
    implementation = _impl,
    local = False,
    attrs = {
      "arch": attr.string(
        mandatory = True,
        values = ["i386", "amd64", "armhf", "arm64"],
        doc = "Architecture for which to build the sysroot",
      ),
      "distro": attr.string(
        mandatory = True,
        values = ["stretch", "buster", "bullseye"],
        doc = "Debian distro (release) to retrieve packages from",
      ),
      "channels": attr.string_list(
        allow_empty = False,
        default = ["main"],
        doc = "Debian channels to retrieve packages from",
      ),
      "packages": attr.string_list(
        mandatory = True,
        allow_empty = False,
        doc = "Packages to extract into the sysroot",
      ),
      "indices_release_url": attr.string(
        default = "https://github.com/DCNick3/debian-listing-archive/releases/download/2021-11-15T204144",
        doc = "Base url of the indices release",
      ),
      "indices_hashes_sha256": attr.string(
        default = "2df766e998b5ec4fc6fc0c5a6198db0e5fbafa289817e1c06d965360a30e1e26",
        doc = "Hash of the hashes.txt corresponding to indices_release_url",
      ),
      "debian_mirror_url": attr.string(
        default = "http://deb.debian.org/debian",
        doc = "Debian mirror to use",
      )
    }
  )

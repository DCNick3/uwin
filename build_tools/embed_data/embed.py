
import argparse
import subprocess
import tempfile
from pathlib import Path
import re
import hashlib
import shutil
from typing import IO

SAFE_IDENTIFIER_REGEX = re.compile(r"[_a-z]+[A-Za-z0-9]*")

parser = argparse.ArgumentParser()

parser.add_argument("files", nargs='+', help="Files to embed", type=Path)
parser.add_argument("-Wl", dest="lflags", nargs='+', help="Linker flags to pass")
parser.add_argument("-l", dest="linker", required=True, help="Linker executable to use")
parser.add_argument("--example-object", required=True, help="Example object file to facilitate target emulation for linker", type=Path)
parser.add_argument("--object", required=True, help="Output object file", type=Path)
parser.add_argument("--cpp", required=True, help="Output C++ file", type=Path)
parser.add_argument("--header", required=True, help="Output C++ header file", type=Path)
parser.add_argument("--namespace", required=True, help="C++ namespace")
parser.add_argument("--embedname", required=True, help="Name for this embed")

args = parser.parse_args()

args.namespace_elements = args.namespace.split("::") if args.namespace.strip() else []

assert all(SAFE_IDENTIFIER_REGEX.match(x) for x in args.namespace_elements)
assert SAFE_IDENTIFIER_REGEX.match(args.embedname)

def hash_file(path):
    sha256_hash = hashlib.sha256()
    with open(path, "rb") as f:
        for byte_block in iter(lambda: f.read(4096), b""):
            sha256_hash.update(byte_block)
    return sha256_hash.hexdigest()

def cescape(s):
  r = ''
  for c in s:
    ordc = ord(c)
    if ordc < 0x20 or ordc >= 0x7f:
      raise RuntimeError("Cannot escape " + s)
    if c == '\\' or c == '"':
      r += "\\" + c
    else:
      r += c
  return r

def genc(args, files, out: IO):
  out.write("#include <cstddef>\n")
  for ns in args.namespace_elements:
    out.write("namespace %s {\n" % ns)

  out.write("""\nstruct FileToc {
  const char* name;             // the file's original name
  const char* data;             // beginning of the file
  std::size_t size;             // length of the file
};\n\n""")

  for linker_name in set(x[1] for x in files):
    symbol_prefix = '_binary_' + linker_name
    begin = symbol_prefix + "_start"
    end = symbol_prefix + "_end"
    out.write("extern \"C\" char %s[];\n" % begin)
    out.write("extern \"C\" char %s[];\n" % end)

  out.write("\nstatic const struct FileToc toc[] = {\n")

  for name, linker_name in files:
    symbol_prefix = '_binary_' + linker_name
    begin = symbol_prefix + "_start"
    end = symbol_prefix + "_end"
    out.write('  {"%s", %s, (std::size_t)(%s - %s) - 1}, ' \
      '/* All files are padded with NUL byte at the end, hence the -1 */\n' \
      % (cescape(name), begin, end, begin))

  out.write("  {nullptr, nullptr, 0},\n")
  out.write("}; // static const struct FileToc toc[]\n")

  out.write("""\nconst struct FileToc* %s_create() {
  return &toc[0];
}\n""" % args.embedname)

  for ns in args.namespace_elements:
    out.write("} // namespace %s\n" % ns)

def genh(args, files, out: IO):
  out.write("""#pragma once
#include <cstddef>\n""")
  for ns in args.namespace_elements:
    out.write("namespace %s {\n" % ns)

  out.write("""\nstruct FileToc {
  const char* name;             // the file's original name
  const char* data;             // beginning of the file
  std::size_t size;             // length of the file
};\n\n""")

  out.write("extern const struct FileToc* %s_create();\n\n" % args.embedname)

  for ns in args.namespace_elements:
    out.write("} // namespace %s\n" % ns)

def link(args):
  with tempfile.TemporaryDirectory() as d:
    d = Path(d)

    files = []
    file: Path
    for file in args.files:
      name = file.name
      name_for_linker = '_'.join(args.namespace_elements + [args.embedname]) + '_' + hash_file(file)

      with open(d / name_for_linker, 'wb') as dst, \
          open(file, 'rb') as src:
        shutil.copyfileobj(src, dst)
        dst.write(b"\x00")
      files.append((name, name_for_linker))
    
    linker_command = [Path(shutil.which(args.linker)).absolute()]
    linker_command += ["-r"]
    linker_command += args.lflags or []
    linker_command += [args.example_object.absolute()]
    linker_command += ["--format=binary"]
    linker_command += list(set(x[1] for x in files))
    linker_command += [
      "--format=default",
      "-o",
      args.object.absolute(),
    ]

    subprocess.check_call(linker_command, cwd=d)

  return files

files = link(args)
with open(args.cpp, 'w') as f:
  genc(args, files, f)
with open(args.header, 'w') as f:
  genh(args, files, f)
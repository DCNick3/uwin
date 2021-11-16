
import argparse
import arpy
import re
import tarfile
import os
import shutil
from pathlib import Path
from contextlib import contextmanager

CONTROL_REGEX = re.compile(r"^control.tar(|\.gz|\.xz)$")
DATA_REGEX    = re.compile(r"^data.tar(|\.gz|\.xz)$")
PACKAGE_NAME_REGEX = re.compile(r"^Package: ([^\n]+)$", re.M)

parser = argparse.ArgumentParser()
parser.add_argument('-o', dest='out', required=True, help='directory in which to extract')
parser.add_argument('-c', dest='clean', action='store_true', help='delete the destination directory before operation')
parser.add_argument('debs', nargs='+', help='Deb files to extract')

args = parser.parse_args()


DEST = Path(args.out).resolve()

if args.clean:
    shutil.rmtree(DEST)
os.makedirs(DEST, exist_ok=True)

def single(it):
    it = iter(it)
    try:
        res = next(it)
    except StopIteration:
        raise RuntimeError("Iterable contained to elements")
    try:
        next(it)
    except StopIteration:
        return res
    raise RuntimeError("Was not single element in the iterable")

@contextmanager
def write_to(path, binary=False):
    full_path = (DEST / path).resolve()
    assert full_path.is_relative_to(DEST) # do not escape me!!
    # I don't care about preserving directory permissions, so it's okay to create them here
    os.makedirs(full_path.parent, exist_ok=True)
    try:
        with open(full_path, 'xb' if binary else 'x') as f:
            yield f
    except FileExistsError:
        raise RuntimeError(f"Tried to open file {path}, but it was already exising. Is the output directory clean? Do two packages have the same files?")
        

def extract_control(control_file):
    tar = tarfile.open(fileobj=control_file)
    def read(filename):
        with tar.extractfile(tar.getmember(filename)) as f:
            return str(f.read(), 'utf8')

    package_name = single(PACKAGE_NAME_REGEX.findall(read('./control')))

    control_dest = Path('debian') / package_name / 'DEBIAN'

    with write_to(control_dest / 'control') as f:
        f.write(read('./control'))
    with write_to(control_dest / 'md5sums') as f:
        f.write(read('./md5sums'))

def extract_data(data_file):
    tar = tarfile.open(fileobj=data_file)
    for member in tar.getmembers():
        if member.isfile():
            with write_to(member.name, True) as dest, \
                    tar.extractfile(member) as src:
                shutil.copyfileobj(src, dest)

def extract_package(package_path):
    with arpy.Archive(package_path) as ar:
        with ar.open(b'debian-binary') as f:
            assert f.read() == b'2.0\n'
        control_filename = single(x for x in ar.namelist() if CONTROL_REGEX.match(str(x, 'utf8')))
        data_filename = single(x for x in ar.namelist() if DATA_REGEX.match(str(x, 'utf8')))
        with ar.open(control_filename) as f:
            extract_control(f)
        with ar.open(data_filename) as f:
            extract_data(f)

for package in args.debs:
    extract_package(package)



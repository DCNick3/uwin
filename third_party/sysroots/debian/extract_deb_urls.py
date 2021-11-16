
import sys
import lzma
import argparse
import json

parser = argparse.ArgumentParser()
parser.add_argument('-i', dest="indices", metavar="Packages.xz", nargs='+', help='Index file to look in (Packages.xz)', required=True)
parser.add_argument('-p', dest="packages", metavar="libpackage-dev", nargs='+', help='Package name to search for', required=True)

args = parser.parse_args()

packages = set(args.packages)

def iter_entries(index):
    with lzma.open(index, 'r') as f:
        buffer = []
        package_name = None
        for line in f:
            line = str(line, 'utf8')
            if line[:9] == 'Package: ':
                package_name = line[9:-1]
            if line.isspace():
                yield package_name, buffer
                buffer = []
            else:
                buffer.append(line)

def parse_control(entries):
    res = dict()
    prev_key = None
    for entry in entries:
        if entry[0].isspace():
            res[prev_key] += '\n' + entry[1:]
            continue
        k, v = entry.split(': ', 1)
        prev_key = k
        res[k] = v[:-1]
    return res

FOUND_PACKAGES = dict()
                
for index in args.indices:
    for package_name, entry in iter_entries(index):
        if package_name in packages:
            entry = parse_control(entry)
            if package_name in FOUND_PACKAGES:
                raise RuntimeError("Duplicate package: " + package_name)
            FOUND_PACKAGES[package_name] = entry

for package in packages:
    if package not in FOUND_PACKAGES:
        raise RuntimeError("Package not found: " + package)

result = []
for package in FOUND_PACKAGES.values():
    result.append({
        "name": package['Package'],
        "filename": package['Filename'],
        "sha256": package['SHA256']
    })

print(json.dumps(result, sort_keys=True, indent=4))


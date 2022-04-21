import argparse
import csv
import sys

errors = []

with open("winerror.csv") as f:
    reader = csv.reader(f)
    next(reader)  # skip header
    for code, slug, desc in reader:
        code = int(code, 10)
        errors.append((code, slug, desc))

parser = argparse.ArgumentParser()
parser.add_argument('error', type=int)

args = parser.parse_args()

for code, slug, desc in errors:
    if code == args.error:
        print('Error {0} (0x{0:08x})'.format(code))
        print(slug)
        print("Description:")
        print(desc)
        break
else:
    print("Error not found in the database =(")
    sys.exit(1)

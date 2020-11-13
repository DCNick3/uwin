import csv

errors = []

with open("winerror.csv") as f:
    reader = csv.reader(f)
    next(reader) # skip header
    for code, slug, desc in reader:
        code = int(code, 10)
        errors.append((code, slug, desc))
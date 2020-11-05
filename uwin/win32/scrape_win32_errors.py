
# usage: python3 scrape_win32_errors.py

import requests
from bs4 import BeautifulSoup
import csv
import sys

URL = "https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/18d8fbe8-a967-4f1c-ae50-99ca8e491d2d" 

r = requests.get(URL)
r.raise_for_status()
text = r.text

soup = BeautifulSoup(text, 'html.parser')
table = soup.find_all("table")
assert len(table) == 1
table = table[0]


data = set()

for row in table.select('tbody > tr'):
    
    t = row.select('td')
    code, slug = t[0].text.strip().split('\n')
    
    # Nice one, M$
    code = int(code.replace('Â\xa0', ''), 16)
    
    desc = t[1].text.strip().replace('\n  ', ' ')
    data.add((code, slug, desc))

def rm_slug(target_slug):
    res = [ (code, slug, desc) for code, slug, desc in data if slug == target_slug ]
    for x in res:
        data.remove(x)
    
rm_slug("NERR_Success")
    
assert len(set(map(lambda x: x[0], data))) == len(data), "Duplicate error code values found"

with open('winerror.csv', 'w', newline='') as f:
    writer = csv.writer(f, quoting=csv.QUOTE_MINIMAL)
    writer.writerow(['code', 'slug', 'desc'])
    for code, slug, desc in sorted(data, key=lambda x: x[0]):
        writer.writerow([code, slug, desc])
        print(desc)

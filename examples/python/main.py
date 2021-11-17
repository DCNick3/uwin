import sys

import requests

print("I am running using", sys.executable)

r = requests.get("http://httpbin.org/get")
r.raise_for_status()

print(r.json())
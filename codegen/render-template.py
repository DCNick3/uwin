
from mako.template import Template

import argparse
import sys
import os

parser = argparse.ArgumentParser()
parser.add_argument('template')
parser.add_argument('output')

args = parser.parse_args()

sys.path.insert(0, os.curdir)

template = Template(filename=args.template)

with open(args.output, 'w') as f:
    f.write(template.render())


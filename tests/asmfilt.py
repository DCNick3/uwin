
import sys

text = sys.stdin.read()
# really simplistic
#
text = '\n'.join([ '; ' + x for x in text.split('\n') ])
text = text.replace('PTR ', '')
text = text.replace(' .', ' ->')
print(text)

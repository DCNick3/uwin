
import sys
import re

HDR_RE = re.compile(r"^\.text:[0-9A-F]{8}", re.MULTILINE)
VAR_RE = re.compile(r"^ [a-zA-Z0-9_]+\s*=\s*[^\n]*\n", re.MULTILINE)
ENDP_RE = re.compile(r"[a-zA-Z0-9_]+\s*endp")
COMMENT_RE = re.compile(r";.*$", re.MULTILINE)
H_RE = re.compile(r"([0-9A-F]+)h")
LABEL_RE = re.compile(r"^ ([a-zA-Z0-9_]+):", re.MULTILINE)
PROC_RE = re.compile(r"^ ([a-zA-Z0-9_]+)\s*proc near", re.MULTILINE)

text = sys.stdin.read()

text = HDR_RE.sub('', text)
text = VAR_RE.sub('', text)
text = COMMENT_RE.sub('', text)
text = ENDP_RE.sub('', text)
text = H_RE.sub('0x\\1', text)
text = LABEL_RE.sub('       ->\\1:', text)
text = PROC_RE.sub(' ->\\1:', text)

text = '\n'.join([ ';' + x.rstrip() for x in text.split('\n') if x.rstrip() ])

text = text.replace('dword ptr', 'DWORD')
text = text.replace('word ptr', 'WORD')
text = text.replace('byte ptr', 'BYTE')
text = text.replace('short ', '->')


print(text)

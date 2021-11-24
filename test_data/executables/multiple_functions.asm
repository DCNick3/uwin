format PE GUI 4.0
entry start

include 'win32a.inc'

section '.code' executable readable

start:
    invoke GetModuleHandleA, 0
    invoke ExitProcess, 0
    ret

section '.idata' import data readable
  library kernel,'kernel32.dll'

  import kernel,\
         ExitProcess,'ExitProcess',\
         GetModuleHandleA, 'GetModuleHandleA'

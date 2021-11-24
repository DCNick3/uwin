format PE GUI 4.0
entry start

include 'win32a.inc'

section '.code' executable readable

start:
    invoke MessageBox,0,txtMessage,txtTitle,0
    invoke ExitProcess,0
    ret

section '.rodata' data readable

txtTitle        db      'funny caption',0
txtMessage      db      'Hello World!',0

section '.idata' import data readable

  library kernel,'kernel32.dll',\
          user,'user32.dll'

  import kernel,\
         ExitProcess,'ExitProcess'

  import user,\
         MessageBox,'MessageBoxA'  

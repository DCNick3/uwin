python
import sys
import pathlib

sys.path.insert(0, str(pathlib.Path().absolute() / 'gdb'))
import uwin_pretty_printers
end
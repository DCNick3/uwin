# MakeELF
ELF reader-writer library for Python3

## Overview

MakeELF is a Python library to parse, modify and create ELF binaries. It
provides following features:

* easy to use, standard Python interface
* reading existing ELF files to Python representation
* modification of every aspect of ELF format structures
* ability to skip any validation to test other parsers for potential errors
* creating new valid ELF files with just one step
* easy serialization of every structure present in ELF file

## Installation

MakeELF could be installed either using PyPI and pip3 tool or manually with help
of setup.py script.

### Install from PyPI

It is as simple as typing:

```shell
pip3 install makeelf
```

### Manual installation

Clone the Github repository and in root of the project execute:

```shell
python3 setup.py install
```

## API

### Creating new object

```Python
from makeelf.elf import *
elf = ELF(e_machine=EM.EM_LKV373A)
print(elf)
```

### Parsing ELF file

Either using helper function:

```Python
elf = ELF.from_file('some.elf')
print(elf)
```

Then if you'd like to have full control on headers, low-level object can be
extracted:

```Python
elf = elf.Elf
```

Or create low-level object manually:

```Python
fd = os.open('some.elf', os.O_RDONLY)
b = os.read(fd, 0xffff)
os.close(fd)

elf, b = Elf32.from_bytes(b)
print(elf)
```

### Saving ELF to file

```Python
fd = os.open('other.elf', os.O_WRONLY | os.O_CREAT | os.O_TRUNC)
os.write(fd, bytes(elf))
os.close(fd)
```

### Adding a section

```Python
data_id = elf.append_section('.data', b'\0\0\0\0', 0xfadd)
```

### Adding a symbol

```Python
elf.append_symbol('NULL', data_id, 0, 4)
```

### Modifying attributes

```Python
elf.Elf.Shdr_table[data_id].sh_flags = int(SHF.SHF_ALLOC)
```

## License

All the software here is licensed under GNU General Public License 3.0,
available in LICENSE file.

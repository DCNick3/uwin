import gdb.printing
import gdb.types
from gdb.types import TypePrinter

import re

def str_pp(base_type):
    class Printer:
        def __init__(self, val):
            self.val = val

        def display_hint(self):
            return "string"
        def to_string(self):
            bases = [x for x in self.val.type.fields() if x.is_base_class]
            assert len(bases) == 1
            base = bases[0]
            return self.val[base] # TODO: handle encodings for narrow strings
    return Printer

def tptr_pp():
    class Printer:
        def __init__(self, val):
            self.val = val

        def to_string(self):
            val = self.val['_value']
            if val.is_optimized_out:
                return '<optimized out>'
            return '0x{:08x}'.format(int(str(val), 0))
    return Printer


TPTR_TP_RE = re.compile(r"^uwin::mem::tptr<(.*), (true|false)>$")
def tptr_tp(type):
    typename = gdb.types.get_basic_type(type).tag
    m = TPTR_TP_RE.match(typename)
    t = m.group(1)
    const = m.group(2)
    if const == 'true':
        res = 'uwin::mem::tcptr<{}>'.format(t)
    else:
        res = 'uwin::mem::tptr<{}>'.format(t)
    return res


def get_const_type_printer(value):
    def ret(type):
        return value
    return ret

class RegexpCollectionTypePrinter(TypePrinter):
    """Class for implementing a collection of regular-expression based pretty-printers.

    Intended usage:

    pretty_printer = RegexpCollectionPrettyPrinter("my_library")
    pretty_printer.add_printer("myclass1", "^myclass1$", MyClass1Printer)
    ...
    pretty_printer.add_printer("myclassN", "^myclassN$", MyClassNPrinter)
    register_pretty_printer(obj, pretty_printer)
    """
    class RegexpSubprinter(TypePrinter):
        def __init__(self, name, regexp, recognize):
            super(RegexpCollectionTypePrinter.RegexpSubprinter, self).__init__(name)
            self.regexp = regexp
            self.recognize = recognize
            self.compiled_re = re.compile(regexp)

    def __init__(self, name):
        super(RegexpCollectionTypePrinter, self).__init__(name)
        self.subprinters = []

    def add_printer(self, name, regexp, recognize):
        self.subprinters.append(self.RegexpSubprinter(name, regexp, recognize))

    def add_const_printer(self, name, regexp, value):
        self.add_printer(name, regexp, get_const_type_printer(value))

    def recognize(self, type):
        # Get the type name.
        typename = gdb.types.get_basic_type(type).tag
        if not typename:
            return None

        # Iterate over table of type regexps to determine
        # if a printer is registered for that type.
        # Return an instantiation of the printer if found.
        for printer in self.subprinters:
         if printer.enabled and printer.compiled_re.search(typename):
             return printer.recognize(type)

        # Cannot find a pretty printer.  Return None.
        return None
    def instantiate(self):
        #print("INSTANTIATE")
        return self

def build_pretty_printer():
    pp = gdb.printing.RegexpCollectionPrettyPrinter("uwin")
    pp.add_printer('str::native', '^uwin::str::detail::generic_str<char, uwin::str::detail::native_tag>$', str_pp('std::string'))
    pp.add_printer('str::narrow', '^uwin::str::detail::generic_str<char, uwin::str::detail::narrow_tag>$', str_pp('std::string'))
    pp.add_printer('str::wide', '^uwin::str::detail::generic_str<char16_t, uwin::str::detail::wide_tag>$', str_pp('std::wstring'))
    pp.add_printer('mem:taddr', TPTR_TP_RE, tptr_pp())
    return pp
def build_type_printer():
    tp = RegexpCollectionTypePrinter("uwin")
    tp.add_const_printer('str::native', '^uwin::str::detail::generic_str<char, uwin::str::detail::native_tag>$', 'uwin::str::native')
    tp.add_const_printer('str::narrow', '^uwin::str::detail::generic_str<char, uwin::str::detail::narrow_tag>$', 'uwin::str::narrow')
    tp.add_const_printer('str::wide', '^uwin::str::detail::generic_str<char16_t, uwin::str::detail::wide_tag>$', 'uwin::str::wide')
    # TODO: tptr type printer seems broken
    tp.add_printer('mem::tptr', TPTR_TP_RE, tptr_tp)
    return tp

print("Hello, gdb python!")
gdb.printing.register_pretty_printer(None, build_pretty_printer())
gdb.types.register_type_printer(None, build_type_printer())

print(gdb.pretty_printers)
print(gdb.type_printers)
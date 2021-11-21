
import sys
from pyparsing import ParseBaseException
import pytest

import def_parser

if __name__ == "__main__":
  sys.exit(pytest.main(sys.argv[1:]))

def check_parse_dict(s, dict_res):
  assert def_parser.export_definition.parseString(s, True).asDict() == dict_res


def test_export_definition_parse():
  check_parse_dict("SasiFun", {
    'entryname': 'SasiFun',
    'ordinal': [],
    'redirection': [],
    'data': []})

  check_parse_dict("SasiFun@12", {
    'entryname': 'SasiFun@12',
    'ordinal': [],
    'redirection': [],
    'data': []})

  check_parse_dict("SasiFun @12", {
    'entryname': 'SasiFun',
    'ordinal': {
      'ordinal_value': 12,
      'export_by_name': True},
    'redirection': [],
    'data': []})

  check_parse_dict("SasiFun @12 NONAME", {
    'entryname': 'SasiFun',
    'ordinal': {
      'ordinal_value': 12,
      'export_by_name': False},
    'redirection': [],
    'data': []})

  check_parse_dict("SasiFun=kernel32.GetLastError", {
    'entryname': 'SasiFun',
    'redirection': {
      'other_module': 'kernel32',
      'exported_name': 'GetLastError'},
    'ordinal': [],
    'data': []})

  check_parse_dict("SasiFun=kernel32.GetLastError @12", {
    'entryname': 'SasiFun',
    'redirection': {
      'other_module': 'kernel32',
      'exported_name': 'GetLastError'},
    'ordinal': {
      'ordinal_value': 12,
      'export_by_name': True},
    'data': []})

  check_parse_dict("SasiFun=kernel32.GetLastError @12 NONAME", {
    'entryname': 'SasiFun',
    'redirection': {
      'other_module': 'kernel32',
      'exported_name': 'GetLastError'},
    'ordinal': {
      'ordinal_value': 12,
      'export_by_name': False},
    'data': []})

  with pytest.raises(ParseBaseException) as excinfo:\
    def_parser.export_definition.parseString("SasiFun DATA", True)
  assert excinfo.value.msg.startswith('DATA must have a value assigned')

  check_parse_dict("SasiFun DATA=\"\\x13\\x37\"", {
    'entryname': 'SasiFun',
    'redirection': [],
    'ordinal': [],
    'data': [b'\x13\x37']})

def test_library_stmt():
  assert def_parser.library_stmt.parseString("LIBRARY \"ws2_32.dll\"", True).asList() == ['LIBRARY', 'ws2_32.dll']

def test_exports_stmt():
  assert def_parser.exports_stmt.parseString("EXPORTS dick\nnick\n", True).asList() == ['EXPORTS', [['dick', [], [], []], ['nick', [], [], []]]]
  # note: no newline at the end of input
  assert def_parser.exports_stmt.parseString("EXPORTS dick\nnick", True).asList() == ['EXPORTS', [['dick', [], [], []], ['nick', [], [], []]]]

  with pytest.raises(ParseBaseException) as excinfo:
    def_parser.exports_stmt.parseString("EXPORTS thicc=libthin.thin DATA=\"1337\"", True)
  assert excinfo.value.msg.startswith('Cannot both export as DATA and perform redirection')

def test_def_file_grammar():
  assert def_parser.def_file.parseString("""; this is a comment

; the previos line is empty
LIBRARY "ws2_32.dll"
EXPORTS
  kick
  nick @12


  prick@12
  sick=12.21

  lick=21.12 @15
  tick=11.22 @22 NONAME

  pick DATA="hello"

""").asList() == [
    [';', ' this is a comment'],
    [';', ' the previos line is empty'],
    ['LIBRARY', 'ws2_32.dll'],
    ['EXPORTS', [
      ['kick', [], [], []],
      ['nick', [], ['@', 12, True], []],
      ['prick@12', [], [], []],
      ['sick', ['=', '12', '.', '21'], [], []],
      ['lick', ['=', '21', '.', '12'], ['@', 15, True], []],
      ['tick', ['=', '11', '.', '22'], ['@', 22, False], []],
      ['pick', [], [], [b'hello']],
    ]]
  ]

  with pytest.raises(ParseBaseException) as excinfo:
    def_parser.def_file.parseString("LIBRARY \"a.dll\"\nLIBRARY \"a.dll\"", True)
  assert excinfo.value.msg.startswith('Exactly one library statement should be present')

  with pytest.raises(ParseBaseException) as excinfo:
    def_parser.def_file.parseString("LIBRARY \"libtest.dll\"\nEXPORTS GetLastError=GetLastErrorInternal", True)
  assert excinfo.value.msg.startswith("Expected end of text")

def test_parse_def_file():
  assert def_parser.parse_def_string("LIBRARY \"libtest.dll\"\nEXPORTS GetLastError") == {
    'library_name': 'libtest.dll',
    'exports': {
      'GetLastError': {'entryname': 'GetLastError', 'export_by_name': True, 'redirect_to': None, 'ordinal': None, 'data': None}
    }
  }


  assert def_parser.parse_def_string("""; this is a comment

; the previos line is empty
LIBRARY "ws2_32.dll"
EXPORTS
  kick
  nick @12


  prick@12
  sick=12.21

  lick=21.12 @15
  tick=11.22 @22 NONAME

  pick DATA="hello"

""") == {
  'library_name': 'ws2_32.dll',
  'exports': {
    'kick': {
      'entryname': 'kick',
      'export_by_name': True,
      'ordinal': None,
      'redirect_to': None,
      'data': None,
    },
    'nick': {
      'entryname': 'nick',
      'export_by_name': True,
      'ordinal': 12,
      'redirect_to': None,
      'data': None,
    },
    'prick@12': {
      'entryname': 'prick@12',
      'export_by_name': True,
      'ordinal': None,
      'redirect_to': None,
      'data': None,
    },
    'sick': {
      'entryname': 'sick',
      'export_by_name': True,
      'ordinal': None,
      'redirect_to': ('12', '21'),
      'data': None,
    },
    'lick': {
      'entryname': 'lick',
      'export_by_name': True,
      'ordinal': 15,
      'redirect_to': ('21', '12'),
      'data': None,
    },
    'tick': {
      'entryname': 'tick',
      'export_by_name': False,
      'ordinal': 22,
      'redirect_to': ('11', '22'),
      'data': None,
    },
    'pick': {
      'entryname': 'pick',
      'export_by_name': True,
      'ordinal': None,
      'redirect_to': None,
      'data': b'hello',
    },
  }
}
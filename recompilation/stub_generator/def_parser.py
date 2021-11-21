
from ast import literal_eval
from io import StringIO
from typing import IO, Tuple
from pyparsing import Char, CharsNotIn, QuotedString, Word, Keyword, Optional, Group, ParserElement, delimitedList, alphanums, nums


# Rrrrr, global functions are evil...
ParserElement.set_default_whitespace_chars(" \t")

quoted_string = QuotedString("\"", "\\")

LIBRARY = Keyword("LIBRARY")
library_name = quoted_string

library_stmt = LIBRARY + library_name.setResultsName("library_name")

EXPORTS = Keyword("EXPORTS")
c_identifier = Word(alphanums + '@_')

NONAME = Keyword("NONAME")
DATA = Keyword("DATA")

redirected_name = Char('=') + c_identifier.setResultsName("other_module") + '.' + c_identifier.setResultsName("exported_name")
ordinal_def = Char("@") + \
  Word(nums).setResultsName("ordinal_value").setParseAction(lambda x: int(x[0])) + \
    Group(Optional(NONAME)) \
      .setResultsName("export_by_name") \
      .setParseAction(lambda x: not x[0])

data_def = Optional(DATA + Optional(Char('=') + QuotedString("\"", "\\", unquoteResults=False))) \
      .addCondition(lambda x: len(list(x)) != 1, message="DATA must have a value assigned (for example 'DATA = \"\\x13\\x37\")", fatal=True) \
      .addParseAction(lambda x: bytes(literal_eval('b' + x[2])) if x else [])

export_definition = (c_identifier.setResultsName("entryname") + \
    Group(Optional(redirected_name)).setResultsName("redirection") + \
    Group(Optional(ordinal_def)).setResultsName("ordinal") + \
    Group(Optional(data_def)).setResultsName("data")) \
  .addCondition(lambda x: not ((not not x['data']) and (not not x['redirection'])),
    message="Cannot both export as DATA and perform redirection", fatal=True)

exports_stmt = EXPORTS + Group(
    delimitedList(Group(Optional(export_definition)), '\n')
      .setParseAction(lambda toks: [x for x in toks if x])
  )

comment_stmt = Char(';') + CharsNotIn('\n')

def_file = delimitedList(Group(Optional(comment_stmt | library_stmt | exports_stmt)), '\n') \
      .setParseAction(lambda toks: [x for x in toks if x]) \
      .addCondition(lambda toks: len([x for x in toks if x[0] == 'LIBRARY']) == 1, message='Exactly one library statement should be present') \
      .addCondition(lambda toks: len([x for x in toks if x[0] == 'EXPORTS']) == 1, message='Exactly one exports statement should be present')

class DefExportsEntry(dict):
  def __init__(self, entryname, ordinal, export_by_name, redirect_to, data):
    dict.__init__(self,
      entryname=entryname,
      ordinal=ordinal,
      export_by_name=export_by_name,
      redirect_to=redirect_to,
      data=data,
    )
  
  @property
  def entryname(self) -> str: return self['entryname']
  @property
  def ordinal(self) -> int: return self['ordinal']
  @property
  def export_by_name(self) -> bool: return self['export_by_name']
  @property
  def redirect_to(self) -> Tuple[str, str]: return self['redirect_to']
  @property
  def data(self) -> Tuple[str, str]: return self['data']

class DefFile(dict):
  def __init__(self, library_name, exports):
    dict.__init__(self,
      library_name=library_name,
      exports=exports,
    )
  
  @property
  def library_name(self) -> str: return self['library_name']
  @property
  def exports(self) -> dict[str, DefExportsEntry]: return self['exports']



def parse_def_string(s: str) -> DefFile:
  parsed = def_file.parseString(s, parseAll=True)

  res = dict()

  for x in parsed:
    if x[0] == ';':
      continue
    elif x[0] == 'LIBRARY':
      res['library_name'] = x[1]
    elif x[0] == 'EXPORTS':
      exports = dict()
      for export in x[1]:
        entryname = export['entryname']
        if entryname in exports:
          raise RuntimeError("Duplicate export: " + entryname)

        exports[entryname] = DefExportsEntry(
          entryname = entryname,
          ordinal = export['ordinal']['ordinal_value'] if export['ordinal'] else None,
          export_by_name = export['ordinal']['export_by_name'] if export['ordinal'] else True,
          redirect_to = (export['redirection']['other_module'], \
            export['redirection']['exported_name']) if export['redirection'] else None,
          data = export['data'][0] if export['data'] else None,
        )
      res['exports'] = exports
  
  return DefFile(**res)
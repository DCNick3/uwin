
#include <iostream>
#include <elfio/elfio_dump.hpp>

int main(int argc, char** argv) {
  using namespace ELFIO;

  if ( argc != 2 ) {
    printf( "Usage: elfdump <file_name>\n" );
    return 1;
  }

  elfio reader;

  if ( !reader.load( argv[1] ) ) {
    printf( "File %s is not found or it is not an ELF file\n", argv[1] );
    return 1;
  }

  dump::header( std::cout, reader );
  dump::section_headers( std::cout, reader );
  dump::segment_headers( std::cout, reader );
  //dump::symbol_tables( std::cout, reader );
  dump::notes( std::cout, reader );
  dump::modinfo( std::cout, reader );
  dump::dynamic_tags( std::cout, reader );
  //dump::section_datas( std::cout, reader );
  //dump::segment_datas( std::cout, reader );
  return 0;
}

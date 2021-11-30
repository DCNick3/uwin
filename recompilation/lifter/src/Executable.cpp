
#include "Executable.h"

#include <glog/logging.h>

#include <exception>
#include <vector>

static std::vector<uint64_t> find_disas_info(ELFIO::elfio const& elfio) {
  for (auto s : elfio.sections) {
    if (s->get_name() == ".uwin_bbaddrs") {
      LOG(INFO) << "Found .uwin_bbaddrs section";
      return {};
      //s->get_data()
    }
  }

  LOG(FATAL) << "Could not find disas info in the executable. Did you run it through disassembler?";
}

Executable::Executable(std::string const& path) {
  if (!_elfio.load(path)) {
    LOG(FATAL) << "Cannot open loaded executable: " << path;
  }

  if (_elfio.get_class() != ELFCLASS32 ||
      _elfio.get_machine() != EM_386)
      LOG(FATAL) << "Unexpected elf class or machine";
  DLOG(INFO) << "Opened executable " << path;

  _disas_addresses = find_disas_info(_elfio);

}

bool Executable::TryReadExecutableByte(uint64_t addr, uint8_t *byte) {
  std::terminate();
}

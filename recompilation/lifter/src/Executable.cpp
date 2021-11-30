
#include "Executable.h"
#include "nlohmann/json_fwd.hpp"

#include <cstdint>
#include <glog/logging.h>
#include <nlohmann/json.hpp>

#include <exception>
#include <optional>
#include <string_view>
#include <utility>
#include <vector>

static std::vector<uint64_t> FindDisasInfo(ELFIO::elfio const& elfio) {
  for (auto s : elfio.sections) {
    if (s->get_name() == ".uwin_bbaddrs") {
      LOG(INFO) << "Found .uwin_bbaddrs section";

      auto data_raw = std::string_view(s->get_data(), s->get_size());

      auto data = nlohmann::json::parse(data_raw);
      
      do {
        if (!data.is_array())
          break;
        if (data[0] != "uwin_disas_v1")
          break;

        std::uint64_t addr = 0;

        std::vector<std::uint64_t> res;
        res.reserve(data.size() + 1);

        for (auto it = /*skip first*/++data.begin(); it != data.end(); ++it) {
          addr += it->get<std::int64_t>();
          res.push_back(addr);
        }

        return std::move(res);
      } while(false);

      LOG(FATAL) << "Cannot parse data in .uwin_bbaddrs section";
      //s->get_data()
    }
  }

  LOG(FATAL) << "Could not find disas info in the executable. Did you run it through disassembler?";
}

static ELFIO::symbol_section_accessor FindSymbolsSection(ELFIO::elfio const& elfio) {
  for (auto sec : elfio.sections) {
    if (sec->get_type() == SHT_SYMTAB) {
      return {elfio, sec};
    }
  }
  LOG(FATAL) << "Cannot find symbols section";
}

Executable::Executable(std::string const& path) {
  if (!_elfio.load(path)) {
    LOG(FATAL) << "Cannot open loaded executable: " << path;
  }

  if (_elfio.get_class() != ELFCLASS32 ||
      _elfio.get_machine() != EM_386)
      LOG(FATAL) << "Unexpected elf class or machine";
  DLOG(INFO) << "Opened executable " << path;

  _disas_addresses = FindDisasInfo(_elfio);

  const auto symbols = FindSymbolsSection(_elfio);

  for (int i = 1; i < symbols.get_symbols_num(); i++) {
    std::string name;
    ELFIO::Elf64_Addr value;
    ELFIO::Elf_Xword size;
    unsigned char bind;
    unsigned char type;
    ELFIO::Elf_Half section_index;
    unsigned char other;

    symbols.get_symbol(i, name, value, size, bind, type, section_index, other);

    if (type == STT_FUNC && size != 0) {
      _fun_name_ranges.emplace(true, value, value + size, name);
    }
  }

  DLOG(INFO) << "Symbols loaded!";
}

std::vector<std::uint64_t> const& Executable::GetDisassembledAddresses() const {
  return _disas_addresses;
}

std::optional<std::pair<std::string, std::uint64_t>> Executable::TryGetFunctionNameAt(std::uint64_t addr) const {
  auto s = _fun_name_ranges.find_ranges(addr);
  if (s.empty())
    return {};
  return std::make_pair(s.top()->mapped(), addr - s.top()->range().get_left());
}

bool Executable::TryReadExecutableByte(uint64_t addr, uint8_t *byte) const {
  std::terminate();
}

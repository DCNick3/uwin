
#pragma once

#include <elfio/elfio.hpp>
#include <beneficii/range_map/range_map.hpp>
#include <optional>

class Executable {
  ELFIO::elfio _elfio;
  std::vector<std::uint64_t> _disas_addresses;
  beneficii::range_map<std::uint64_t, std::string> _fun_name_ranges;

public:
  // Try to read an executable byte of memory. Returns `true` of the byte
  // at address `addr` is executable and readable, and updates the byte
  // pointed to by `byte` with the read value.
  bool TryReadExecutableByte(uint64_t addr, uint8_t *byte) const;

  std::optional<std::pair<std::string, std::uint64_t>> TryGetFunctionNameAt(std::uint64_t addr) const;

  std::vector<std::uint64_t> const& GetDisassembledAddresses() const;

  Executable(std::string const& path);
};


#pragma once

#include <elfio/elfio.hpp>

class Executable {
  ELFIO::elfio _elfio;

public:
  // Try to read an executable byte of memory. Returns `true` of the byte
  // at address `addr` is executable and readable, and updates the byte
  // pointed to by `byte` with the read value.
  bool TryReadExecutableByte(uint64_t addr, uint8_t *byte);

  Executable(std::string const& path);
};

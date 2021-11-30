
#include "Executable.h"

#include <glog/logging.h>

#include <exception>

Executable::Executable(std::string const& path) {
  if (!_elfio.load(path)) {
    LOG(FATAL) << "Cannot open loaded executable: " << path;
  }
  DLOG(INFO) << "Opened executable " << path;
}

bool Executable::TryReadExecutableByte(uint64_t addr, uint8_t *byte) {
  std::terminate();
}


#include <llvm/IR/Module.h>

#include <set>

namespace remill::uwin {
  void OptimizeUwinModule(llvm::Module &module, std::set<std::string> const& lifted_functions);
}

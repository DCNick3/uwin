
#include "Optimize.h"

#include <llvm/IR/Dominators.h>
#include <llvm/IR/Verifier.h>
#include <llvm/Transforms/IPO.h>
#include <llvm/Transforms/IPO/AlwaysInliner.h>
#include <llvm/Analysis/ProfileSummaryInfo.h>
#include <llvm/Analysis/BranchProbabilityInfo.h>
#include <llvm/Analysis/LoopInfo.h>
#include <llvm/Analysis/TargetLibraryInfo.h>
#include <llvm/Analysis/PostDominators.h>
#include <llvm/Analysis/AliasAnalysis.h>
#include <llvm/Analysis/BlockFrequencyInfo.h>
#include <llvm/Analysis/AssumptionCache.h>

#include <glog/logging.h>

namespace remill::uwin {

// copy all lifted functions into a new module, discard the old one
class RemillSemanticsStripperPass : public llvm::PassInfoMixin<RemillSemanticsStripperPass> {
  std::set<std::string> const& _lifted_functions;
public:
  RemillSemanticsStripperPass(std::set<std::string> const& lifted_functions)
    : _lifted_functions(lifted_functions)
    {}

  llvm::PreservedAnalyses run(llvm::Module &M, llvm::ModuleAnalysisManager &) {
    LOG(INFO) << "Running RemillSemanticsStripperPass";

    std::vector<llvm::StringRef> rm_list;

    // TODO: this is a hack that might remove somthing that should not have been removed
    M.getGlobalVariable("llvm.compiler.used")->eraseFromParent();

    // this function keeps references to all the intrinsics
    auto intr = M.getFunction("__remill_intrinsics");
    if (intr != nullptr) {
      intr->replaceAllUsesWith(llvm::UndefValue::get(intr->getType()));
      intr->eraseFromParent();
    }

    for (auto& gl : M.globals()) {
      if (gl.getName().startswith("ISEL_")) {
        rm_list.emplace_back(gl.getName());
      }
    }

    for (auto gl_name : rm_list) {
      auto gl = M.getGlobalVariable(gl_name);
      gl->replaceAllUsesWith(llvm::UndefValue::get(gl->getType()));
      gl->eraseFromParent();
    }

    /*rm_list.clear();

    for (auto& fun : M.functions()) {
      if (_lifted_functions.find(fun.getName().str()) == _lifted_functions.end())
        rm_list.emplace_back(fun.getName());
      //else
      //  LOG(INFO) << "Not removing " << fun.getName().str();
    }

    for (auto const& fun_name : rm_list) {
      auto fun = M.getFunction(fun_name);
      bool droppable = true;
      for (auto const& user : fun->users()) {
        //user->dump();
        droppable &= user->isDroppable();
      }
      if (droppable) {
        fun->eraseFromParent();
      }
      else {
        if (!fun->isDeclaration()) {
          LOG(WARNING) << "Can't remove " << fun->getName().str() << ", as it has some uses";
        }
        //fun->eraseFromParent();
      }
    }*/

    return {}; // invalidate everything (because doing otherwise is hard)
  }
  static bool isRequired() { return true; }
};

void OptimizeUwinModule(llvm::Module &module, std::set<std::string> const& lifted_functions) {
  llvm::ModuleAnalysisManager MAM;
  llvm::FunctionAnalysisManager FAM;

  MAM.registerPass([&](){return llvm::PassInstrumentationAnalysis();});
  MAM.registerPass([&](){return llvm::InnerAnalysisManagerProxy<llvm::FunctionAnalysisManager, llvm::Module>(FAM);});
  MAM.registerPass([&](){return llvm::ProfileSummaryAnalysis();});
  MAM.registerPass([&](){return llvm::VerifierAnalysis();});

  FAM.registerPass([&](){return llvm::PassInstrumentationAnalysis();});
  FAM.registerPass([&](){return llvm::AssumptionAnalysis();});
  FAM.registerPass([&](){return llvm::BlockFrequencyAnalysis();});
  FAM.registerPass([&](){return llvm::BranchProbabilityAnalysis();});
  FAM.registerPass([&](){return llvm::LoopAnalysis();});
  FAM.registerPass([&](){return llvm::DominatorTreeAnalysis();});
  FAM.registerPass([&](){return llvm::PostDominatorTreeAnalysis();});
  FAM.registerPass([&](){return llvm::TargetLibraryAnalysis();});
  FAM.registerPass([&](){return llvm::AAManager();});

  llvm::ModulePassManager MPM;

  // inline all intrinsics and semantics
  // don't want lifetime information as not to bloat the bitcode
  MPM.addPass(llvm::AlwaysInlinerPass(false));
  MPM.addPass(RemillSemanticsStripperPass(lifted_functions));
  MPM.addPass(llvm::VerifierPass());

  MPM.run(module, MAM);
}

} // namespace remill::uwin

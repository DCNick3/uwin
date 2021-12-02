
#include "Optimize.h"
#include "remill/BC/Util.h"

#include <llvm/IR/Dominators.h>
#include <llvm/IR/Verifier.h>
#include <llvm/Analysis/ProfileSummaryInfo.h>
#include <llvm/Analysis/BranchProbabilityInfo.h>
#include <llvm/Analysis/LoopInfo.h>
#include <llvm/Analysis/TargetLibraryInfo.h>
#include <llvm/Analysis/PostDominators.h>
#include <llvm/Analysis/AliasAnalysis.h>
#include <llvm/Analysis/BlockFrequencyInfo.h>
#include <llvm/Analysis/AssumptionCache.h>
#include <llvm/Analysis/MemorySSA.h>

#include <llvm/Transforms/Utils/Cloning.h>

#include <llvm/Transforms/IPO.h>
#include <llvm/Transforms/IPO/AlwaysInliner.h>

#include <llvm/Transforms/Scalar/SROA.h>
#include <llvm/Transforms/Scalar/EarlyCSE.h>
#include <llvm/Transforms/Scalar/SimplifyCFG.h>
#include <llvm/Transforms/Scalar/Reassociate.h>
#include <llvm/Transforms/Scalar/SCCP.h>

#include <glog/logging.h>

namespace remill::uwin {

namespace {
// copy all lifted functions into a new module, discard the old one
class RemillSemanticsStripperPass : public llvm::PassInfoMixin<RemillSemanticsStripperPass> {
  std::set<std::string> const& _lifted_functions;
public:
  RemillSemanticsStripperPass(std::set<std::string> const& lifted_functions)
    : _lifted_functions(lifted_functions)
    {}

  llvm::PreservedAnalyses run(llvm::Module &M, llvm::ModuleAnalysisManager &) {
    LOG(INFO) << "Running RemillSemanticsStripperPass";

    // clone the module
    auto M1 = llvm::CloneModule(M);

    LOG(INFO) << "Cleaning module...";
    // clean the module, same as in the destructor
    // (Is it still safe to use it?)
    M.dropAllReferences();
    M.getGlobalList().clear();
    M.getFunctionList().clear();
    M.getAliasList().clear();
    M.getIFuncList().clear();
    M.getNamedMDList().clear();

    LOG(INFO) << "Moving all the functions into the clean module...";
    for (auto& fun_name : _lifted_functions) {
      auto fun = M1->getFunction(fun_name);
      CHECK(fun != nullptr);
      remill::MoveFunctionIntoModule(fun, &M);
    }

    return {}; // invalidate everything (because doing otherwise is hard)
  }
  static bool isRequired() { return true; }
};

auto BuildFunctionSimplificationPipeline() {
  llvm::FunctionPassManager FPM;

  // Kind of based on -O1 pipeline, but I have no idea of what I am doing
  FPM.addPass(llvm::SROA()); // scalar replacement of aggregates (wut?)
  FPM.addPass(llvm::EarlyCSEPass(true)); // common subexpression elimination

  // invokePeepholeEPCallbacks ???

  FPM.addPass(llvm::SimplifyCFGPass());
  FPM.addPass(llvm::ReassociatePass()); // is it useful?

  // no loops here, as there are no loops yet, all instrs live in their own world

  FPM.addPass(llvm::SCCPPass()); // constant propagation and merging

  return FPM;
}

}

void OptimizeUwinModule(llvm::Module &module, std::set<std::string> const& lifted_functions) {
  llvm::ModuleAnalysisManager MAM;
  llvm::FunctionAnalysisManager FAM;

  MAM.registerPass([&](){return llvm::InnerAnalysisManagerProxy<llvm::FunctionAnalysisManager, llvm::Module>(FAM);});
  MAM.registerPass([&](){return llvm::PassInstrumentationAnalysis();});
  MAM.registerPass([&](){return llvm::ProfileSummaryAnalysis();});
  MAM.registerPass([&](){return llvm::VerifierAnalysis();});

  FAM.registerPass([&](){return llvm::OuterAnalysisManagerProxy<llvm::ModuleAnalysisManager, llvm::Function>(MAM);});
  FAM.registerPass([&](){return llvm::PassInstrumentationAnalysis();});
  FAM.registerPass([&](){return llvm::AssumptionAnalysis();});
  FAM.registerPass([&](){return llvm::BlockFrequencyAnalysis();});
  FAM.registerPass([&](){return llvm::BranchProbabilityAnalysis();});
  FAM.registerPass([&](){return llvm::LoopAnalysis();});
  FAM.registerPass([&](){return llvm::DominatorTreeAnalysis();});
  FAM.registerPass([&](){return llvm::PostDominatorTreeAnalysis();});
  FAM.registerPass([&](){return llvm::TargetLibraryAnalysis();});
  FAM.registerPass([&](){return llvm::AAManager();});
  FAM.registerPass([&](){return llvm::TargetIRAnalysis();});
  FAM.registerPass([&](){return llvm::MemorySSAAnalysis();});

  llvm::ModulePassManager MPM;

  // inline all intrinsics and semantics
  // don't want lifetime information as not to bloat the bitcode
  MPM.addPass(llvm::AlwaysInlinerPass(false));
  
  // TODO: focus only on lifted functions
  MPM.addPass(llvm::createModuleToFunctionPassAdaptor(
    BuildFunctionSimplificationPipeline()
  ));
  //MPM.addPass(RemillSemanticsStripperPass(lifted_functions));
  MPM.addPass(llvm::VerifierPass());

  MPM.run(module, MAM);
}

} // namespace remill::uwin

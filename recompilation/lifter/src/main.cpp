

#include <cassert>
#include <gflags/gflags.h>
#include <glog/logging.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Dominators.h>
#include <llvm/IR/Verifier.h>
#include <llvm/Linker/Linker.h>
#include <llvm/Support/CommandLine.h>
#include <llvm/Transforms/IPO.h>
#include <llvm/Transforms/IPO/AlwaysInliner.h>
#include <llvm/Analysis/ProfileSummaryInfo.h>
#include <llvm/Analysis/BranchProbabilityInfo.h>
#include <llvm/Analysis/LoopInfo.h>
#include <llvm/Analysis/TargetLibraryInfo.h>
#include <llvm/Analysis/PostDominators.h>
#include <llvm/Analysis/AliasAnalysis.h>

#include <memory>
#include <remill/Arch/Arch.h>
#include <remill/Arch/Name.h>
#include <remill/BC/IntrinsicTable.h>
#include <remill/BC/Lifter.h>
#include <remill/BC/Optimizer.h>
#include <remill/BC/Util.h>
#include <remill/OS/OS.h>

#include "recompilation/lifter/uwin_intrinsics.h"
#include "Executable.h"
#include "TraceManager.h"
#include "remill_vendor/TraceLifter.h"
#include "remill_vendor/InstructionLifter.h"

#include <fstream>
#include <iostream>
#include <map>
#include <string_view>

DEFINE_string(elf_filename, "",
              "Filename of loaded elf file to lift");

DEFINE_string(ir_out, "", "Path to file where the LLVM IR should be saved.");
DEFINE_string(bc_out, "",
              "Path to file where the LLVM bitcode should be "
              "saved.");

std::unique_ptr<llvm::Module> LoadIntrinsics(llvm::LLVMContext* context) {
  auto intrinsics = remill::uwin_intrinsics_create();
  assert(intrinsics->data != nullptr);
  auto data = std::string_view(intrinsics->data, intrinsics->size);
  return remill::LoadModuleFromMemory(context, data);
}

int main(int argc, char** argv) {
  google::SetVersionString(":shrug:");
  google::ParseCommandLineFlags(&argc, &argv, true);
  google::InitGoogleLogging(argv[0]);

  FLAGS_stderrthreshold = 0;

  google::SetCommandLineOption("arch", "x86");

  if (FLAGS_elf_filename.empty()) {
    LOG(FATAL) << "Please, specify --elf_filename";
  }

  //addOccurrence

  auto& opts = llvm::cl::getRegisteredOptions();
  //opts["opt-bisect-limit"]->addOccurrence(0, "opt-bisect-limit", "-1");
  llvm::cl::PrintOptionValues();

  //llvm::cl::

  llvm::LLVMContext context;
  auto arch = remill::Arch::Build(&context, remill::OSName::kOSWindows,
                                  remill::ArchName::kArchX86);

  std::unique_ptr<llvm::Module> module(remill::LoadArchSemantics(arch));

  Executable executable(FLAGS_elf_filename);

  SimpleTraceManager manager(module.get(), executable);
  remill::IntrinsicTable intrinsics(module);
  remill::uwin::InstructionLifter inst_lifter(arch, intrinsics);
  remill::uwin::TraceLifter trace_lifter(inst_lifter, manager);

  LOG(INFO) << "Ready to start lifting!";

  // Lift all discoverable traces with addresses taken from file
  
  // TODO: lift ALL possible addresses from the file
  // this basically implements the superset disassembly
  
  for (auto const& trace : manager.traces)
    CHECK(trace_lifter.Lift(trace.first));

  // Optimize the module, but with a particular focus on only the functions
  // that we actually lifted.
  //remill::OptimizationGuide guide = {};
  //guide.eliminate_dead_stores = true;
  //remill::OptimizeModule(arch, module, manager.GetDeclaredTraces(), guide);


  // Create a new module in which we will move all the lifted functions. Prepare
  // the module for code of this architecture, i.e. set the data layout, triple,
  // etc.
  // std::unique_ptr<llvm::Module> intermediate_module =
  //     std::make_unique<llvm::Module>("lifted_code", context);

  // arch->PrepareModuleDataLayout(intermediate_module);

  // Move the lifted code into a new module. This module will be much smaller
  // because it won't be bogged down with all of the semantics definitions.
  // This is a good JITing strategy: optimize the lifted code in the semantics
  // module, move it to a new module, instrument it there, then JIT compile it.
  // for (auto &lifted_entry : manager.traces) {
  //   CHECK(lifted_entry.second.lifted); // assert it is lifted
  //   remill::MoveFunctionIntoModule(lifted_entry.second.function, intermediate_module.get());
  // }

  LOG(INFO) << "Lifting done. Now we inline all the semantics and intrinsics...";

  auto intrinsics_module = LoadIntrinsics(&context);
  // trick LLVM to think the targets are the same (it does not matter anyway)
  intrinsics_module->setDataLayout(module->getDataLayout());
  intrinsics_module->setTargetTriple(module->getTargetTriple());

  // load the uwin-specific intrinsics implementations
  llvm::Linker::linkModules(*module, std::move(intrinsics_module));

  llvm::ModuleAnalysisManager MAM;
  llvm::FunctionAnalysisManager FAM;

  MAM.registerPass([&](){return llvm::PassInstrumentationAnalysis();});
  MAM.registerPass([&](){return llvm::InnerAnalysisManagerProxy<llvm::FunctionAnalysisManager, llvm::Module>(FAM);});
  MAM.registerPass([&](){return llvm::ProfileSummaryAnalysis();});

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

  MPM.run(*module, MAM);

  LOG(INFO) << "Happy!";

  // create inliner with really low threshold
  // this (theoretically) should inline only alwaysinline functions

  //guide.slp_vectorize = false;
  //guide.loop_vectorize = false;
  //guide.eliminate_dead_stores = false;
  //guide.verify_input = false;

  //remill::OptimizeBareModule(intrinsics_module, guide);

  int ret = EXIT_SUCCESS;

  // remove the (now inlined) intrinsics and trace functions, not to pollute the global namespace
  // also mark all functions with uwtable attribute to allow C++ exceptions to pass through
  // {
  //   std::vector<std::string> rmnames;
  //   for (auto &fun : intrinsics_module->functions()) {
  //     auto nm = fun.getName().str();
  //     if (nm.rfind("__remill_", 0) == 0) {
  //       rmnames.emplace_back(std::move(nm));
  //     }

  //     fun.addFnAttr(llvm::Attribute::UWTable);
  //   }
  //   for (auto& nm : rmnames) {
  //     auto fun = intrinsics_module->getFunction(nm);
  //     if (fun->uses().empty())
  //     {}//fun->eraseFromParent();
  //     else {
  //       if (!fun->isDeclaration())
  //         LOG(WARNING) << "Can't remove " << nm << ", as it has some uses";
  //       else {
  //         LOG(ERROR) << "Intrinsic " << nm << " does not have implementation";
  //         ret = EXIT_FAILURE;
  //       }
  //     }
  //   }
  // }


  if (!FLAGS_ir_out.empty()) {
    if (!remill::StoreModuleIRToFile(module.get(), FLAGS_ir_out, true)) {
      LOG(ERROR) << "Could not save LLVM IR to " << FLAGS_ir_out;
      ret = EXIT_FAILURE;
    }
  }
  if (!FLAGS_bc_out.empty()) {
    if (!remill::StoreModuleToFile(module.get(), FLAGS_bc_out, true)) {
      LOG(ERROR) << "Could not save LLVM bitcode to " << FLAGS_bc_out;
      ret = EXIT_FAILURE;
    }
  }

  return ret;

}


#include <cassert>
#include <gflags/gflags.h>
#include <glog/logging.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/Linker/Linker.h>
#include <llvm/Support/CommandLine.h>
#include <llvm/IR/Verifier.h>
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
  //trace_lifter.Lift(uint64_t addr)

  // Optimize the module, but with a particular focus on only the functions
  // that we actually lifted.
  remill::OptimizationGuide guide = {};
  guide.eliminate_dead_stores = true;
  remill::OptimizeModule(arch, module, manager.GetDeclaredTraces(), guide);


  // Create a new module in which we will move all the lifted functions. Prepare
  // the module for code of this architecture, i.e. set the data layout, triple,
  // etc.
  std::unique_ptr<llvm::Module> intermediate_module =
      std::make_unique<llvm::Module>("lifted_code", context);

  arch->PrepareModuleDataLayout(intermediate_module);

  // Move the lifted code into a new module. This module will be much smaller
  // because it won't be bogged down with all of the semantics definitions.
  // This is a good JITing strategy: optimize the lifted code in the semantics
  // module, move it to a new module, instrument it there, then JIT compile it.
  for (auto &lifted_entry : manager.traces) {
    CHECK(lifted_entry.second.lifted); // assert it is lifted
    remill::MoveFunctionIntoModule(lifted_entry.second.function, intermediate_module.get());
  }

  auto dispatcher_fun = llvm::Function::Create(arch->LiftedFunctionType(),
                                                 llvm::GlobalValue::LinkageTypes::ExternalLinkage,
                                               "uwin_xcute_remill_dispatch_recompiled",
                                               *intermediate_module);
 {

    auto args_ptr = dispatcher_fun->arg_begin();
    auto state = args_ptr++;
    auto pc = args_ptr++;
    auto mem = args_ptr++;

    std::vector<llvm::Value*> args;
    args.push_back(state);
    args.push_back(pc);
    args.push_back(mem);

    llvm::IRBuilder<> builder(context);

    auto block = llvm::BasicBlock::Create(context, "entry", dispatcher_fun);


    auto abort = llvm::BasicBlock::Create(context, "abort");
    {
      builder.SetInsertPoint(abort);

      auto unk = intermediate_module->getOrInsertFunction("uwin_xcute_remill_dispatch_unknown", arch->LiftedFunctionType());

      builder.CreateRet(builder.CreateCall(unk, args));
    }

    {
      builder.SetInsertPoint(block);

      auto sw = builder.CreateSwitch(pc, abort, manager.traces.size());
      for (auto bb : manager.traces) {
        std::stringstream hexbbss;
        hexbbss << std::hex << bb.first;
        std::string hexbb = hexbbss.str();

        auto call = llvm::BasicBlock
        ::Create(context,
                 "call_" + hexbb);

        builder.SetInsertPoint(call);

        auto fun = intermediate_module->getFunction(manager.TraceName(bb.first));
                          //arch->LiftedFunctionType());
        auto newmem = builder.CreateCall(fun, args);

        // do not expose the sub_* functions
        // TODO: do it only when compiling release?
        // TODO: is Private any better than InternalLinkage? Does it give any optimization chances?
        fun->setLinkage(llvm::GlobalValue::InternalLinkage);

        builder.CreateRet(newmem);

        call->insertInto(dispatcher_fun);

        sw->addCase(builder.getInt32(bb.first), call);
      }
    }
    abort->insertInto(dispatcher_fun);
  }


  auto intrinsics_module = LoadIntrinsics(&context);

  // Here we do some voodoo magic. We link modules with different target
  // triples and data layouts. But this is okay, as there are no pointers
  // inside remill-generated code besides State& and Memory*. They are fine,
  // as they are using fixed-size types, ensuring no padding in-between
  // structure elements, and avoiding arch-specific types like long double.
  intermediate_module->setDataLayout(intrinsics_module->getDataLayout());
  intermediate_module->setTargetTriple(intrinsics_module->getTargetTriple());
  llvm::Linker::linkModules(*intrinsics_module, std::move(intermediate_module));

  guide.slp_vectorize = false;
  guide.loop_vectorize = false;
  guide.eliminate_dead_stores = false;
  guide.verify_input = false;

  remill::OptimizeBareModule(intrinsics_module, guide);

  int ret = EXIT_SUCCESS;

  // remove the (now inlined) intrinsics and trace functions, not to pollute the global namespace
  // also mark all functions with uwtable attribute to allow C++ exceptions to pass through
  {
    std::vector<std::string> rmnames;
    for (auto &fun : intrinsics_module->functions()) {
      auto nm = fun.getName().str();
      if (nm.rfind("__remill_", 0) == 0) {
        rmnames.emplace_back(std::move(nm));
      }

      fun.addFnAttr(llvm::Attribute::UWTable);
    }
    for (auto& nm : rmnames) {
      auto fun = intrinsics_module->getFunction(nm);
      if (fun->uses().empty())
      {}//fun->eraseFromParent();
      else {
        if (!fun->isDeclaration())
          LOG(WARNING) << "Can't remove " << nm << ", as it has some uses";
        else {
          LOG(ERROR) << "Intrinsic " << nm << " does not have implementation";
          ret = EXIT_FAILURE;
        }
      }
    }
  }


  if (!FLAGS_ir_out.empty()) {
    if (!remill::StoreModuleIRToFile(intrinsics_module.get(), FLAGS_ir_out, true)) {
      LOG(ERROR) << "Could not save LLVM IR to " << FLAGS_ir_out;
      ret = EXIT_FAILURE;
    }
  }
  if (!FLAGS_bc_out.empty()) {
    if (!remill::StoreModuleToFile(intrinsics_module.get(), FLAGS_bc_out, true)) {
      LOG(ERROR) << "Could not save LLVM bitcode to " << FLAGS_bc_out;
      ret = EXIT_FAILURE;
    }
  }

  return ret;

}
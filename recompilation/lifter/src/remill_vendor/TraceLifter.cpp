/*
 * Copyright (c) 2020 Trail of Bits, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */


#include <set>
#include <sstream>

#include "TraceLifter.h"
#include "InstructionLifter.h"

#include <remill/Arch/Instruction.h>
#include <remill/Arch/Arch.h>
#include <remill/BC/IntrinsicTable.h>
#include <remill/BC/Util.h>
#include <llvm/IR/Instructions.h>

#include <glog/logging.h>

// I don't have time for your riddles!
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

namespace remill::uwin {
namespace {}  // namespace

TraceManager::~TraceManager(void) {}

// Return an already lifted trace starting with the code at address
// `addr`.
llvm::Function *TraceManager::GetLiftedTraceDeclaration(uint64_t) {
  return nullptr;
}

// Return an already lifted trace starting with the code at address
// `addr`.
llvm::Function *TraceManager::GetLiftedTraceDefinition(uint64_t) {
  return nullptr;
}

// Apply a callback that gives the decoder access to multiple virtual
// targets of this instruction (indirect call or jump).
void TraceManager::ForEachDevirtualizedTarget(
    const Instruction &,
    std::function<void(uint64_t, DevirtualizedTargetKind)>) {

  // Must be extended.
}

// Figure out the name for the trace starting at address `addr`.
std::string TraceManager::TraceName(uint64_t addr) {
  std::stringstream ss;
  ss << "sub_" << std::hex << addr;
  return ss.str();
}

namespace {

using DecoderWorkList = std::set<uint64_t>;  // For ordering.

}  // namespace

class TraceLifter::Impl {
 public:
  Impl(InstructionLifter *inst_lifter_, TraceManager *manager_);

  // Lift one or more traces starting from `addr`. Calls `callback` with each
  // lifted trace.
  bool Lift(uint64_t addr,
            std::function<void(uint64_t, llvm::Function *)> callback);

  // Reads the bytes of an instruction at `addr` into `state.inst_bytes`.
  bool ReadInstructionBytes(uint64_t addr);

  // Return an already lifted trace starting with the code at address
  // `addr`.
  //
  // NOTE: This is guaranteed to return either `nullptr`, or a function
  //       within `module`.
  llvm::Function *GetLiftedTraceDeclaration(uint64_t addr);

  // Return an already lifted trace starting with the code at address
  // `addr`.
  //
  // NOTE: This is guaranteed to return either `nullptr`, or a function
  //       within `module`.
  llvm::Function *GetLiftedTraceDefinition(uint64_t addr);

  llvm::BasicBlock *GetOrCreateBlock(uint64_t block_pc) {
    auto &block = blocks[block_pc];
    if (!block) {
      block = llvm::BasicBlock::Create(context, "", func);
      auto target = GetLiftedTraceDeclaration(block_pc);
      if (target == nullptr)
        return block;
      AddTerminatingTailCall(block, target);
    }
    return block;
  }

  llvm::BasicBlock *GetOrCreateBranchTakenBlock(void) {
    return GetOrCreateBlock(inst.branch_taken_pc);
  }

  llvm::BasicBlock *GetOrCreateBranchNotTakenBlock(void) {
    CHECK(inst.branch_not_taken_pc != 0);
    return GetOrCreateBlock(inst.branch_not_taken_pc);
  }

  llvm::BasicBlock *GetOrCreateNextBlock(void) {
    return GetOrCreateBlock(inst.next_pc);
  }

  const Arch *const arch;
  InstructionLifter &inst_lifter;
  const remill::IntrinsicTable *intrinsics;
  llvm::LLVMContext &context;
  llvm::Module *const module;
  const uint64_t addr_mask;
  TraceManager &manager;

  llvm::Function *func;
  llvm::BasicBlock *block;
  llvm::SwitchInst *switch_inst;
  const size_t max_inst_bytes;
  std::string inst_bytes;
  Instruction inst;
  Instruction delayed_inst;
  std::map<uint64_t, llvm::BasicBlock *> blocks;
};

TraceLifter::Impl::Impl(InstructionLifter *inst_lifter_, TraceManager *manager_)
    : arch(inst_lifter_->impl->arch),
      inst_lifter(*inst_lifter_),
      intrinsics(inst_lifter.impl->intrinsics),
      context(inst_lifter.impl->word_type->getContext()),
      module(intrinsics->async_hyper_call->getParent()),
      addr_mask(arch->address_size >= 64 ? ~0ULL
                                         : (~0ULL >> arch->address_size)),
      manager(*manager_),
      func(nullptr),
      block(nullptr),
      switch_inst(nullptr),
      max_inst_bytes(arch->MaxInstructionSize()) {

  inst_bytes.reserve(max_inst_bytes);
}

// Return an already lifted trace starting with the code at address
// `addr`.
llvm::Function *TraceLifter::Impl::GetLiftedTraceDeclaration(uint64_t addr) {
  auto func = manager.GetLiftedTraceDeclaration(addr);
  if (!func || func->getParent() == module) {
    return func;
  }

  return nullptr;
}

// Return an already lifted trace starting with the code at address
// `addr`.
llvm::Function *TraceLifter::Impl::GetLiftedTraceDefinition(uint64_t addr) {
  auto func = manager.GetLiftedTraceDefinition(addr);
  if (!func || func->getParent() == module) {
    return func;
  }

  CHECK_EQ(&(func->getContext()), &context);

  auto func_type = llvm::dyn_cast<llvm::FunctionType>(
      RecontextualizeType(func->getFunctionType(), context));

  // Handle the different module situation by declaring the trace in
  // this module to be external, with the idea that it will link to
  // another module.
  auto extern_func = module->getFunction(func->getName());
  if (!extern_func || extern_func->getFunctionType() != func_type) {
    extern_func = llvm::Function::Create(
        func_type, llvm::GlobalValue::ExternalLinkage, func->getName(), module);

  } else if (extern_func->isDeclaration()) {
    extern_func->setLinkage(llvm::GlobalValue::ExternalLinkage);
  }

  return extern_func;
}

TraceLifter::~TraceLifter(void) {}

TraceLifter::TraceLifter(InstructionLifter *inst_lifter_,
                         TraceManager *manager_)
    : impl(new Impl(inst_lifter_, manager_)) {}

void TraceLifter::NullCallback(uint64_t, llvm::Function *) {}

// Reads the bytes of an instruction at `addr` into `inst_bytes`.
bool TraceLifter::Impl::ReadInstructionBytes(uint64_t addr) {
  inst_bytes.clear();
  for (size_t i = 0; i < max_inst_bytes; ++i) {
    const auto byte_addr = (addr + i) & addr_mask;
    if (byte_addr < addr) {
      break;  // 32- or 64-bit address overflow.
    }
    uint8_t byte = 0;
    if (!manager.TryReadExecutableByte(byte_addr, &byte)) {
      DLOG(WARNING) << "Couldn't read executable byte at " << std::hex
                    << byte_addr << std::dec;
      break;
    }
    inst_bytes.push_back(static_cast<char>(byte));
  }
  return !inst_bytes.empty();
}

// Lift one or more traces starting from `addr`.
bool TraceLifter::Lift(
    uint64_t addr, std::function<void(uint64_t, llvm::Function *)> callback) {
  return impl->Lift(addr, callback);
}

// Lift one or more traces starting from `addr`.
bool TraceLifter::Impl::Lift(
    uint64_t addr_, std::function<void(uint64_t, llvm::Function *)> callback) {
  auto addr = addr_ & addr_mask;
  if (addr < addr_) {  // Address is out of range.
    LOG(ERROR) << "Trace address " << std::hex << addr_ << " is too big"
               << std::dec;
    return false;
  }

  // Reset the lifting state.
  blocks.clear();
  inst_bytes.clear();
  func = nullptr;
  switch_inst = nullptr;
  block = nullptr;
  inst.Reset();
  delayed_inst.Reset();

  // Get a trace head that the manager knows about, or that we
  // will eventually tell the trace manager about.
  auto get_trace_decl = [=](uint64_t addr) -> llvm::Function * {
    if (auto trace = GetLiftedTraceDeclaration(addr)) {
      return trace;
    }

    return nullptr;
  };
  
  const auto trace_addr = addr;

  // Already lifted.
  func = GetLiftedTraceDefinition(trace_addr);
  if (func) {
    return true;
  }

  DLOG(INFO) << "Lifting trace at address " << std::hex << trace_addr
              << std::dec;

  func = get_trace_decl(trace_addr);
  blocks.clear();

  if (!func || !func->isDeclaration()) {
    const auto trace_name = manager.TraceName(trace_addr);
    func = DeclareLiftedFunction(module, trace_name);
  }

  CHECK(func->isDeclaration());

  // Fill in the function, and make sure the block with all register
  // variables jumps to the block that will contain the first instruction
  // of the trace.
  CloneBlockFunctionInto(func);
  auto state_ptr = NthArgument(func, kStatePointerArgNum);

  if (auto entry_block = &(func->front())) {
    auto pc = LoadProgramCounterArg(func);
    auto next_pc_ref = inst_lifter.LoadRegAddress(entry_block, state_ptr,
                                                  kNextPCVariableName);

    // Initialize `NEXT_PC`.
    (void) new llvm::StoreInst(pc, next_pc_ref, entry_block);

    auto &block = blocks[trace_addr];
    if (!block)
      block = llvm::BasicBlock::Create(context, "", func);

    // Branch to the first basic block.
    llvm::BranchInst::Create(block, entry_block);
  }

  do {
    // Decode instruction.
    const auto inst_addr = trace_addr;

    block = GetOrCreateBlock(inst_addr);
    switch_inst = nullptr;

    // We have already lifted this instruction block.
    CHECK(block->empty());
    
    // No executable bytes here.
    if (!ReadInstructionBytes(inst_addr)) {
      AddTerminatingTailCall(block, intrinsics->missing_block);
      break;
    }

    inst.Reset();

    (void) arch->DecodeInstruction(inst_addr, inst_bytes, inst);

    auto lift_status = inst_lifter.LiftIntoBlock(inst, block, state_ptr);
    if (kLiftedInstruction != lift_status) {
      AddTerminatingTailCall(block, intrinsics->error);
      break;
    }

    // Connect together the basic blocks.
    switch (inst.category) {
      case Instruction::kCategoryInvalid:
      case Instruction::kCategoryError:
        AddTerminatingTailCall(block, intrinsics->error);
        break;

      case Instruction::kCategoryNormal:
      case Instruction::kCategoryNoOp:
        llvm::BranchInst::Create(GetOrCreateNextBlock(), block);
        break;

      // Direct jumps could either be local or could be tail-calls. In the
      // case of a tail call, we'll assume that the trace manager contains
      // advanced knowledge of this, and so when we go to make a block for
      // the targeted instruction, we'll either tail call to the target
      // trace, or we'll just extend out the current trace. Either way, no
      // sacrifice in correctness is made.
      case Instruction::kCategoryDirectJump:
        llvm::BranchInst::Create(GetOrCreateBranchTakenBlock(), block);
        break;

      case Instruction::kCategoryIndirectJump: {
        AddTerminatingTailCall(block, intrinsics->jump);
        break;
      }

      case Instruction::kCategoryAsyncHyperCall:
        AddCall(block, intrinsics->async_hyper_call);
        goto check_call_return;

      case Instruction::kCategoryIndirectFunctionCall: {
        const auto fall_through_block =
            llvm::BasicBlock::Create(context, "", func);

        const auto ret_pc_ref =
            LoadReturnProgramCounterRef(fall_through_block);
        const auto next_pc_ref =
            LoadNextProgramCounterRef(fall_through_block);
        llvm::IRBuilder<> ir(fall_through_block);
        ir.CreateStore(ir.CreateLoad(ret_pc_ref), next_pc_ref);
        ir.CreateBr(GetOrCreateBranchNotTakenBlock());

        AddCall(block, intrinsics->function_call);
        llvm::BranchInst::Create(fall_through_block, block);
        block = fall_through_block;
        break;
      }

      case Instruction::kCategoryConditionalIndirectFunctionCall: {
        auto taken_block = llvm::BasicBlock::Create(context, "", func);
        auto not_taken_block = GetOrCreateBranchNotTakenBlock();
        const auto orig_not_taken_block = not_taken_block;

        llvm::BranchInst::Create(taken_block, not_taken_block,
                                  LoadBranchTaken(block), block);

        AddCall(taken_block, intrinsics->function_call);

        const auto ret_pc_ref = LoadReturnProgramCounterRef(taken_block);
        const auto next_pc_ref = LoadNextProgramCounterRef(taken_block);
        llvm::IRBuilder<> ir(taken_block);
        ir.CreateStore(ir.CreateLoad(ret_pc_ref), next_pc_ref);
        ir.CreateBr(orig_not_taken_block);
        block = orig_not_taken_block;
        break;
      }

      // In the case of a direct function call, we try to handle the
      // pattern of a call to the next PC as a way of getting access to
      // an instruction pointer. It is the case where a call to the next
      // PC could also be something more like a call to a `noreturn` function
      // and that is OK, because either a user of the trace manager has
      // already told us that the next PC is a trace head (and we'll pick
      // that up when trying to lift it), or we'll just have a really big
      // trace for this function without sacrificing correctness.
      case Instruction::kCategoryDirectFunctionCall: {
      direct_func_call:
        // in case of a call branch_not_taken_pc tells the return address
        // branch_taken_pc tells the call target
        if (inst.branch_not_taken_pc != inst.branch_taken_pc) {
          auto target_trace = get_trace_decl(inst.branch_taken_pc);
          if (target_trace)
            AddCall(block, target_trace);
          else
            AddCall(block, intrinsics->missing_block);
        }

        const auto ret_pc_ref = LoadReturnProgramCounterRef(block);
        const auto next_pc_ref = LoadNextProgramCounterRef(block);
        llvm::IRBuilder<> ir(block);
        ir.CreateStore(ir.CreateLoad(ret_pc_ref), next_pc_ref);
        ir.CreateBr(GetOrCreateBranchNotTakenBlock());

        break;
      }

      case Instruction::kCategoryConditionalDirectFunctionCall: {
        if (inst.branch_not_taken_pc == inst.branch_taken_pc) {
          goto direct_func_call;
        }

        auto taken_block = llvm::BasicBlock::Create(context, "", func);
        auto not_taken_block = GetOrCreateBranchNotTakenBlock();
        const auto orig_not_taken_block = not_taken_block;

        llvm::BranchInst::Create(taken_block, not_taken_block,
                                  LoadBranchTaken(block), block);

        auto target_trace = get_trace_decl(inst.branch_taken_pc);

        AddCall(taken_block, intrinsics->function_call);
        AddCall(taken_block, target_trace);

        const auto ret_pc_ref = LoadReturnProgramCounterRef(taken_block);
        const auto next_pc_ref = LoadNextProgramCounterRef(taken_block);
        llvm::IRBuilder<> ir(taken_block);
        ir.CreateStore(ir.CreateLoad(ret_pc_ref), next_pc_ref);
        ir.CreateBr(orig_not_taken_block);
        block = orig_not_taken_block;
        break;
      }

      // Lift an async hyper call to check if it should do the hypercall.
      // If so, it will jump to the `do_hyper_call` block, otherwise it will
      // jump to the block associated with the next PC. In the case of the
      // `do_hyper_call` block, we assign it to `state.block`, then go
      // to `check_call_return` to add the hyper call into that block,
      // checking if the hyper call returns to the next PC or not.
      //
      // TODO(pag): Delay slots?
      case Instruction::kCategoryConditionalAsyncHyperCall: {
        auto do_hyper_call = llvm::BasicBlock::Create(context, "", func);
        llvm::BranchInst::Create(do_hyper_call, GetOrCreateNextBlock(),
                                  LoadBranchTaken(block), block);
        block = do_hyper_call;
        AddCall(block, intrinsics->async_hyper_call);
        goto check_call_return;
      }

      check_call_return:
        do {
          auto pc = LoadProgramCounter(block);
          auto ret_pc = llvm::ConstantInt::get(inst_lifter.impl->word_type,
                                                inst.next_pc);

          llvm::IRBuilder<> ir(block);
          auto eq = ir.CreateICmpEQ(pc, ret_pc);
          auto unexpected_ret_pc =
              llvm::BasicBlock::Create(context, "", func);
          ir.CreateCondBr(eq, GetOrCreateNextBlock(), unexpected_ret_pc);
          AddTerminatingTailCall(unexpected_ret_pc,
                                  intrinsics->missing_block);
        } while (false);
        break;

      case Instruction::kCategoryFunctionReturn:
        AddTerminatingTailCall(block, intrinsics->function_return);
        break;

      case Instruction::kCategoryConditionalFunctionReturn: {
        auto taken_block = llvm::BasicBlock::Create(context, "", func);
        auto not_taken_block = GetOrCreateBranchNotTakenBlock();
        const auto orig_not_taken_block = not_taken_block;

        llvm::BranchInst::Create(taken_block, not_taken_block,
                                  LoadBranchTaken(block), block);

        AddTerminatingTailCall(taken_block, intrinsics->function_return);
        block = orig_not_taken_block;
        break;
      }

      case Instruction::kCategoryConditionalBranch: {
        auto taken_block = GetOrCreateBranchTakenBlock();
        auto not_taken_block = GetOrCreateBranchNotTakenBlock();

        llvm::BranchInst::Create(taken_block, not_taken_block,
                                  LoadBranchTaken(block), block);
        break;
      }

      
      case Instruction::kCategoryConditionalIndirectJump: {
        auto taken_block = llvm::BasicBlock::Create(context, "", func);
        auto not_taken_block = GetOrCreateBranchNotTakenBlock();
        const auto orig_not_taken_block = not_taken_block;

        llvm::BranchInst::Create(taken_block, not_taken_block,
                                  LoadBranchTaken(block), block);

        AddTerminatingTailCall(taken_block, intrinsics->jump);
        block = orig_not_taken_block;
        break;
      }
      default:
        LOG(FATAL) << "Unsupported control flow: " << inst.category;
    }
  } while (false);

  for (auto &block : *func) {
    if (!block.getTerminator()) {
      AddTerminatingTailCall(&block, intrinsics->missing_block);
    }
  }

  callback(trace_addr, func);
  manager.SetLiftedTraceDefinition(trace_addr, func);

  return true;
}

}  // namespace remill::uwin

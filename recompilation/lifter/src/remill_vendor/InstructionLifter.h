/*
 * Copyright (c) 202 Trail of Bits, Inc.
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

#pragma once

#include <cstdint>
#include <memory>
#include <string_view>
#include <unordered_map>
#include <glog/logging.h>

#include <llvm/ADT/SmallVector.h>
#include <llvm/IR/BasicBlock.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/DataLayout.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/IntrinsicInst.h>
#include <llvm/IR/LegacyPassManager.h>
#include <llvm/IR/Metadata.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/Operator.h>
#include <llvm/IR/Type.h>
#include <llvm/Support/raw_ostream.h>
#include <llvm/Transforms/Scalar.h>
#include <llvm/Transforms/Utils/Cloning.h>
#include <llvm/Transforms/Utils/ValueMapper.h>

#include <functional>
#include <ios>
#include <set>
#include <sstream>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include <remill/Arch/Arch.h>
#include <remill/Arch/Instruction.h>
#include <remill/Arch/Name.h>
#include <remill/BC/ABI.h>
#include <remill/BC/Compat/DataLayout.h>
#include <remill/BC/IntrinsicTable.h>
#include <remill/BC/Util.h>
#include <remill/OS/OS.h>

namespace llvm {
class Argument;
class ConstantInt;
class Function;
class Module;
class GlobalVariable;
class LLVMContext;
class IntegerType;
class BasicBlock;
class Value;
}  // namespace llvm

namespace remill {

class Arch;
class Instruction;
class IntrinsicTable;
class Operand;
class OperandExpression;
} // namespace remill

namespace remill::uwin {
class TraceLifter;

enum LiftStatus {
  kLiftedInvalidInstruction,
  kLiftedUnsupportedInstruction,
  kLiftedLifterError,
  kLiftedUnknownISEL,
  kLiftedMismatchedISEL,
  kLiftedInstruction
};

// Wraps the process of lifting an instruction into a block. This resolves
// the intended instruction target to a function, and ensures that the function
// is called with the appropriate arguments.
class InstructionLifter {
 public:
  virtual ~InstructionLifter(void);

  inline InstructionLifter(const std::unique_ptr<const Arch> &arch_,
                           const IntrinsicTable &intrinsics_)
      : InstructionLifter(arch_.get(), &intrinsics_) {}

  inline InstructionLifter(const Arch *arch_, const IntrinsicTable &intrinsics_)
      : InstructionLifter(arch_, &intrinsics_) {}

  InstructionLifter(const Arch *arch_, const IntrinsicTable *intrinsics_);

  // Lift a single instruction into a basic block. `is_delayed` signifies that
  // this instruction will execute within the delay slot of another instruction.
  virtual LiftStatus LiftIntoBlock(Instruction &inst, llvm::BasicBlock *block,
                                   llvm::Value *state_ptr,
                                   bool is_delayed = false);

  // Lift a single instruction into a basic block. `is_delayed` signifies that
  // this instruction will execute within the delay slot of another instruction.
  LiftStatus LiftIntoBlock(Instruction &inst, llvm::BasicBlock *block,
                           bool is_delayed = false);

  // Load the address of a register.
  llvm::Value *LoadRegAddress(llvm::BasicBlock *block, llvm::Value *state_ptr,
                              std::string_view reg_name) const;

  // Load the value of a register.
  llvm::Value *LoadRegValue(llvm::BasicBlock *block, llvm::Value *state_ptr,
                            std::string_view reg_name) const;

  // Clear out the cache of the current register values/addresses loaded.
  void ClearCache(void) const;

 protected:
  friend class TraceLifter;

  // Lift an operand to an instruction.
  virtual llvm::Value *LiftOperand(Instruction &inst, llvm::BasicBlock *block,
                                   llvm::Value *state_ptr, llvm::Argument *arg,
                                   Operand &op);

  // Lift a register operand to a value.
  virtual llvm::Value *
  LiftShiftRegisterOperand(Instruction &inst, llvm::BasicBlock *block,
                           llvm::Value *state_ptr, llvm::Argument *arg,
                           Operand &reg);

  // Lift a register operand to a value.
  virtual llvm::Value *LiftRegisterOperand(Instruction &inst,
                                           llvm::BasicBlock *block,
                                           llvm::Value *state_ptr,
                                           llvm::Argument *arg, Operand &reg);

  // Lift an immediate operand.
  virtual llvm::Value *LiftImmediateOperand(Instruction &inst,
                                            llvm::BasicBlock *block,
                                            llvm::Argument *arg, Operand &op);

  // Lift an expression operand.
  virtual llvm::Value *LiftExpressionOperand(Instruction &inst,
                                             llvm::BasicBlock *block,
                                             llvm::Value *state_ptr,
                                             llvm::Argument *arg, Operand &op);

  // Lift an expression operand.
  virtual llvm::Value *
  LiftExpressionOperandRec(Instruction &inst, llvm::BasicBlock *block,
                           llvm::Value *state_ptr, llvm::Argument *arg,
                           const OperandExpression *op);

  // Lift an indirect memory operand to a value.
  virtual llvm::Value *
  LiftAddressOperand(Instruction &inst, llvm::BasicBlock *block,
                     llvm::Value *state_ptr, llvm::Argument *arg, Operand &mem);

  // Return a register value, or zero.
  llvm::Value *
  LoadWordRegValOrZero(llvm::BasicBlock *block, llvm::Value *state_ptr,
                       std::string_view reg_name, llvm::ConstantInt *zero);

 private:
  InstructionLifter(const InstructionLifter &) = delete;
  InstructionLifter(InstructionLifter &&) noexcept = delete;
  InstructionLifter(void) = delete;

  class Impl;

  const std::unique_ptr<Impl> impl;
};

class InstructionLifter::Impl {
 public:
  Impl(const Arch *arch_, const IntrinsicTable *intrinsics_);

  // Architecture being used for lifting.
  const Arch *const arch;

  // Machine word type for this architecture.
  llvm::IntegerType *const word_type;

  // Set of intrinsics.
  const IntrinsicTable *const intrinsics;

  // Cache of looked up registers inside of `last_func`.
  std::unordered_map<std::string, llvm::Value *> reg_ptr_cache;

  // The function into which we're lifting. If This gets out of date, we
  // clear out `reg_ptr_cache`.
  llvm::Function *last_func{nullptr};

  llvm::Module *const module;
  llvm::Function *const invalid_instruction;
  llvm::Function *const unsupported_instruction;
};

}  // namespace remill::uwin

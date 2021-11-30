
#pragma once

#include <cassert>

#include <remill/BC/TraceLifter.h>

#include "Executable.h"

class SimpleTraceManager : public remill::TraceManager {
 public:
  ~SimpleTraceManager() override = default;

  explicit SimpleTraceManager(llvm::Module *module, Executable &executable);

  std::string TraceName(uint64_t addr) override;

protected:
  // Called when we have lifted, i.e. defined the contents, of a new trace.
  // The derived class is expected to do something useful with this.
   void SetLiftedTraceDefinition(uint64_t addr,
                                 llvm::Function *lifted_func) override;

   // Get a declaration for a lifted trace. The idea here is that a derived
   // class might have additional global info available to them that lets
   // them declare traces ahead of time. In order to distinguish between
   // stuff we've lifted, and stuff we haven't lifted, we allow the lifter
   // to access "defined" vs. "declared" traces.
   //
   // NOTE: This is permitted to return a function from an arbitrary module.
   llvm::Function *GetLiftedTraceDeclaration(uint64_t addr) override;

   // Get a definition for a lifted trace.
   //
   // Used by TraceLifter to check whether the trace was already lifted (only)
   //
   // NOTE: This is permitted to return a function from an arbitrary module.
   llvm::Function *GetLiftedTraceDefinition(uint64_t addr) override;

   // Try to read an executable byte of memory. Returns `true` of the byte
   // at address `addr` is executable and readable, and updates the byte
   // pointed to by `byte` with the read value.
   bool TryReadExecutableByte(uint64_t addr, uint8_t *byte) override;

 public:
  struct Trace {
    llvm::Function* function{nullptr};
    bool lifted{false};
  };

  std::unordered_map<std::uint64_t, llvm::Function *> GetDeclaredTraces();

  Executable& _executable;
  std::unordered_map<uint64_t, Trace> traces;
};

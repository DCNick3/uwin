#include "TraceManager.h"

#include <remill/BC/Util.h>
#include <sstream>

void SimpleTraceManager::SetLiftedTraceDefinition(uint64_t addr,
                                                  llvm::Function *lifted_func) {
  auto &trace = traces[addr];
  assert(trace.function == lifted_func || trace.function == nullptr);
  trace.function = lifted_func;
  trace.lifted = true;
}
llvm::Function *SimpleTraceManager::GetLiftedTraceDeclaration(uint64_t addr) {
  auto trace_it = traces.find(addr);
  if (trace_it != traces.end()) {
    return trace_it->second.function;
  } else {
    return nullptr;
  }
}
llvm::Function *SimpleTraceManager::GetLiftedTraceDefinition(uint64_t addr) {
  auto trace_it = traces.find(addr);
  if (trace_it != traces.end() && trace_it->second.lifted) {
    return trace_it->second.function;
  } else {
    return nullptr;
  }
}
bool SimpleTraceManager::TryReadExecutableByte(uint64_t addr, uint8_t *byte) {
  return _executable.TryReadExecutableByte(addr, byte);
}
std::map<std::uint64_t, llvm::Function *>
SimpleTraceManager::GetDeclaredTraces() {
  decltype(GetDeclaredTraces()) res;
  for (auto &trace : traces) {
    if (trace.second.lifted) {
      res.emplace(trace.first, trace.second.function);
    }
  }
  return res;
}

std::string SimpleTraceManager::TraceName(uint64_t addr) { 
  constexpr std::string_view prefix = "recomp_";
  
  auto name = _executable.TryGetFunctionNameAt(addr);
  
  std::stringstream ss;
  ss << prefix;
  if (name) {
    ss << name->first << "+0x" << std::hex << name->second << "_";
  }
  ss << std::hex << "0x" << addr;

  return ss.str();
}

SimpleTraceManager::SimpleTraceManager(llvm::Module *module,
                                       Executable &executable)
      : _executable(executable) 
{
  for (auto const& addr : _executable.GetDisassembledAddresses())
  {
    traces[addr].function = remill::DeclareLiftedFunction(module, TraceName(addr));
  }
}

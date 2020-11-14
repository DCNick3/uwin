
<%!
import codeutils
import json

def cstr(s):
    return json.dumps(s)
%>

<%
class_name = f"{dll_name}_iface"
%>

#include "win32/dll/${dll_name}_iface.h"
#include "win32/dll/stub_exception.h"
#include "ctx/dll.h"
#include <utility>

namespace uwin {
    namespace win32 {
        namespace dll {
        % for i, fun in funs:
            ${fun.proto(class_name)} {
                throw stub_exception("${dll_name}::${fun.name}");
            }

            void ${class_name}::${fun.name}_raw_call(uwin::xcute::remill::State *st) {
                ## here we do conversion of parameters
                ${codeutils.gen_remill_entry(fun)}
            }

            uwin::xcute::remill::Memory *${class_name}::${fun.name}_remill_entry(uwin::xcute::remill::State *st,
                std::uint32_t pc, uwin::xcute::remill::Memory* memory) {

                xcute::remill::get_process_ctx(st)._dll->get_${dll_name}().${fun.name}_raw_call(st);
                return memory;
            }
        % endfor
            mem::taddr::tvalue ${class_name}::resolve(std::string& name) const {
                if (false) {}
                % for i, fun in funs:
                else if (name == ${cstr(fun.name)})
                    return ${hex(i)};
                % endfor
                else
                    std::terminate();
            }
        }
    }
}

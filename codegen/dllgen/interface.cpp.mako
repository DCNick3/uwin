
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
#include "log/log.h"
#include <utility>

namespace uwin::win32::dll {
% for i, fun in funs:
    ${fun.proto(class_name)} {
        throw stub_exception("${dll_name}::${fun.name}");
    }

    void ${class_name}::${fun.name}_raw_call(uwin::xcute::remill::StateEx& st) {
        ## here we do conversion of parameters
        _current_thread = st.thread_ctx;
        log::trace("${dll_name}!${fun.name}(...)");
${codeutils.gen_remill_entry(fun)}
    }

    /* static */ uwin::xcute::remill::Memory *${class_name}::${fun.name}_remill_entry(uwin::xcute::remill::StateEx& st,
        std::uint32_t pc, uwin::xcute::remill::Memory* memory) {

        st.process_ctx->_dll.get_${dll_name}().${fun.name}_raw_call(st);
        return memory;
    }
% endfor
    [[nodiscard]] mem::tcaddr ${class_name}::try_resolve(const std::string& name) const {
        if (false) {}
        % for i, fun in funs:
        else if (name == ${cstr(fun.name)})
            return mem::tcaddr(${hex(i)});
        % endfor
        else
            return mem::tcaddr(0);
    }
}

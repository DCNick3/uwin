uwin_sources(NAME win32-ldr
        PORTABLE
        module_loader.cpp
            microsoft_pe.cpp
            module.cpp
        DEPENDS dllgen kaitai-rt)
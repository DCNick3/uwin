uwin_sources(NAME win32-ldr
        PORTABLE
            module_loader.cpp
            microsoft_pe.cpp
            target_module.cpp
            module_table.cpp
        DEPENDS dllgen)
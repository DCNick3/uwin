

dllgen(DEFDIR ${CMAKE_CURRENT_LIST_DIR}/defs
        OUTFILES DLLGEN_FILES
        DEFS
            USER32
            KERNEL32
            DDRAW
        )

message("DLLGEN_FILES: ${DLLGEN_FILES}")

uwin_sources(NAME win32-dll
        PORTABLE
        native_module.cpp
        stub_exception.cpp
        USER32/impl.cpp
        KERNEL32/impl.cpp
        KERNEL32/heap.cpp
        KERNEL32/virtual.cpp
        KERNEL32/nls.cpp
        DDRAW/impl.cpp
        ${DLLGEN_FILES}

        DEPENDS win32 win32-ldr dllgen util)

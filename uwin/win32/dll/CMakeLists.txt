

dllgen(DEFDIR ${CMAKE_CURRENT_LIST_DIR}/defs
        OUTFILES DLLGEN_FILES
        DEFS
            TEST
            USER32)

message("DLLGEN_FILES: ${DLLGEN_FILES}")

uwin_sources(NAME win32-dll
        PORTABLE base.cpp
        TEST/impl.cpp
        USER32/impl.cpp
        ${DLLGEN_FILES}

        DEPENDS win32 dllgen)



dllgen(DEFDIR ${CMAKE_CURRENT_LIST_DIR}/defs
        OUTFILES DLLGEN_FILES
        DEFS TEST)

message("DLLGEN_FILES: ${DLLGEN_FILES}")

uwin_sources(NAME win32-dll
        PORTABLE base.cpp stub_exception.cpp ${DLLGEN_FILES}

        DEPENDS win32)

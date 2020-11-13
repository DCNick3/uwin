
set(ERROR_CODE_GEN_H ${CMAKE_CURRENT_BINARY_DIR}/error_code_generated.h)
set(ERROR_CODE_GEN_CXX ${CMAKE_CURRENT_BINARY_DIR}/error_code_generated.cpp)


#add_custom_command(OUTPUT ${ERROR_CODE_GEN_H} ${ERROR_CODE_GEN_CXX}
#        COMMAND python3 ${CMAKE_CURRENT_LIST_DIR}/gen_error_codes.py ${ERROR_CODE_GEN_H} ${ERROR_CODE_GEN_CXX}
#        DEPENDS gen_error_codes.py winerror.csv)

add_mako_codegen(TEMPLATE error_code_generated.h.mako
        OUTPUT "${ERROR_CODE_GEN_H}")
add_mako_codegen(TEMPLATE error_code_generated.cpp.mako
        OUTPUT "${ERROR_CODE_GEN_CXX}")

uwin_sources(NAME win32
        PORTABLE error.cpp ${ERROR_CODE_GEN_H} ${ERROR_CODE_GEN_CXX})

add_subdirectory(dll)


include(PythonVirtualEnv)

add_requirements_to_virtualenv(Mako==1.1.3 MarkupSafe==1.1.1 ply==3.11 pycparser==2.19 pycparserext==2020.1)

message("Codegen python: ${LOCAL_PYTHON_EXECUTABLE}")

set(CODEGEN_RENDER_TEMPLATE_PY "${CMAKE_CURRENT_LIST_DIR}/../codegen/render-template.py")
set(CODEGEN_DLLGEN_PY "${CMAKE_CURRENT_LIST_DIR}/../codegen/dllgen/main.py")

#add_custom_target(codegen)

function(add_mako_codegen)
    set(options)
    set(oneValueArgs TEMPLATE OUTPUT)
    set(multiValueArgs)
    cmake_parse_arguments(ARG "${options}" "${oneValueArgs}"
        "${multiValueArgs}" ${ARGN} )

    if (NOT ARG_TEMPLATE OR NOT ARG_OUTPUT)
        message(FATAL_ERROR "Missing either TEMPLATE or OUTPUT argument")
    endif()

    add_custom_command(
            OUTPUT "${ARG_OUTPUT}"
            DEPENDS "${CODEGEN_RENDER_TEMPLATE_PY}" "${ARG_TEMPLATE}"
            COMMAND "${LOCAL_PYTHON_EXECUTABLE}" "${CODEGEN_RENDER_TEMPLATE_PY}" "${ARG_TEMPLATE}" "${ARG_OUTPUT}"
            WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
    )

    #message("Adding mako template ${ARG_TEMPLATE} in ${CMAKE_CURRENT_SOURCE_DIR}")

    #add_dependencies(codegen )
endfunction()

# quite a dumb function. Should not be called multiple times, as it generates dispatch routine, which is considered
# to be global
function(dllgen)
    set(options)
    set(oneValueArgs DEFDIR OUTFILES)
    set(multiValueArgs DEFS)
    cmake_parse_arguments(ARG "${options}" "${oneValueArgs}"
            "${multiValueArgs}" ${ARGN} )

    if (NOT ARG_DEFS OR NOT ARG_DEFDIR OR NOT ARG_OUTFILES)
        message(FATAL_ERROR "Missing DEFS, OUTFILES or DEFDIR argument")
    endif()

    list(TRANSFORM ARG_DEFS PREPEND "${ARG_DEFDIR}/")
    list(TRANSFORM ARG_DEFS APPEND ".h")

    message("def files: ${ARG_DEFS}")

    execute_process(COMMAND "${LOCAL_PYTHON_EXECUTABLE}" "${CODEGEN_DLLGEN_PY}" --print_out_files
            "${CMAKE_BINARY_DIR}/uwin" ${ARG_DEFS}
            RESULT_VARIABLE ret
            OUTPUT_VARIABLE GENFILES_LIST
            OUTPUT_STRIP_TRAILING_WHITESPACE)
    if(NOT ret EQUAL "0")
        message(FATAL_ERROR "Failed to get dllgen output files")
    endif()

    #list(TRANSFORM GENFILES_LIST PREPEND "${CMAKE_BINARY_DIR}/")

    message("genfiles: ${GENFILES_LIST}")

    add_custom_command(OUTPUT ${GENFILES_LIST}
            COMMAND "${LOCAL_PYTHON_EXECUTABLE}" "${CODEGEN_DLLGEN_PY}"
            "${CMAKE_BINARY_DIR}/uwin" ${ARG_DEFS}
            DEPENDS ${ARG_DEFS})

    add_custom_target(dllgen-target
            DEPENDS ${GENFILES_LIST})

    add_library(dllgen INTERFACE)

    add_dependencies(dllgen dllgen-target)

    set(${ARG_OUTFILES} ${GENFILES_LIST} PARENT_SCOPE)
endfunction()

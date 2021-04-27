
include(PythonVirtualEnv)

set(LIFT_HLP_DIR "${CMAKE_SOURCE_DIR}/codegen/lift-hlp")
set(LIFT_HLP_PY "${LIFT_HLP_DIR}/main.py")

add_requirements_txt_to_virtualenv("${LIFT_HLP_DIR}/requirements.txt")

function(uwin_entry)
    set(options)
    set(singleValueArgs NAME EXE)
    set(multiValueArgs)
    cmake_parse_arguments(PARSE_ARGV 0 "ARG" "${options}" "${singleValueArgs}" "${multiValueArgs}")

    if (NOT ARG_NAME OR NOT ARG_EXE)
        message(FATAL_ERROR "Missing either NAME or EXE argument")
    endif()

    set(lifted_dir "${CMAKE_CURRENT_BINARY_DIR}/lifted/")
    set(lifted_obj "${lifted_dir}/${ARG_NAME}-lifted.o")

    add_custom_command(
            OUTPUT "${lifted_obj}"
            DEPENDS "${LIFT_HLP_PY}" "${ARG_EXE}"
            COMMAND ${CMAKE_COMMAND} -E make_directory "${lifted_dir}" &&
                "${LOCAL_PYTHON_EXECUTABLE}" "${LIFT_HLP_PY}" "--silent" "${ARG_EXE}" "${lifted_obj}"
            WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
    )

    add_executable(uwin-${ARG_NAME} ${lifted_obj})
    target_link_libraries(uwin-${ARG_NAME} uwin-main uwin)
endfunction()
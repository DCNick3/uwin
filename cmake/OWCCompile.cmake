function(owc_compile)
    set(options)
    set(singleValueArgs OUTPUT)
    set(multiValueArgs SOURCES)
    cmake_parse_arguments(PARSE_ARGV 0 "ARG" "${options}" "${singleValueArgs}" "${multiValueArgs}")

    if (NOT ARG_OUTPUT OR NOT ARG_SOURCES)
        message(FATAL_ERROR "Missing either OUTPUT or SOURCES argument")
    endif()

    set(watcom_root "${CONAN_OPENWATCOM-V2_ROOT}")
    # set(owc_build_dir "${CMAKE_BINARY_DIR}/owc_build/")

    # ${CMAKE_COMMAND} -E make_directory "${owc_build_dir}" &&
    add_custom_command(
            OUTPUT "${ARG_OUTPUT}"
            DEPENDS "${SOURCES}"
            COMMAND ${CMAKE_COMMAND} -E env
                "PATH=${watcom_root}/binl64:$PATH"
                "WATCOM=${watcom_root}"
                "EDPATH=${watcom_root}/eddat"
                "WIPFC=${watcom_root}/wipfc"
                "INCLUDE=${watcom_root}/h:${watcom_root}/h/nt"
                "${watcom_root}/binl64/owcc" "-bwin95" "${ARG_SOURCES}" "-o" "${ARG_OUTPUT}"
    )
    # WORKING_DIRECTORY "${owc_build_dir}"
endfunction()
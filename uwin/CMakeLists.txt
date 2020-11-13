

set(UWIN_POSIX TRUE)
set(UWIN_LINUX TRUE)

# this is a meta-target to add includes and compiler definitions.
# probably want to add libraries here
add_library(uwin-base INTERFACE)

target_include_directories(uwin-base INTERFACE ${CMAKE_CURRENT_LIST_DIR} ${CMAKE_CURRENT_BINARY_DIR})

# TODO: make it into a normal package
target_include_directories(uwin-base INTERFACE /home/dcnick3/trash/homm3-switch/lifting/remill/)

add_library(uwin STATIC)

target_include_directories(uwin PUBLIC ${CMAKE_CURRENT_LIST_DIR} ${CMAKE_CURRENT_BINARY_DIR})
# TODO: make it into a normal package
target_include_directories(uwin PUBLIC /home/dcnick3/trash/homm3-switch/lifting/remill/)

function(uwin_sources)
    set(options)
    set(singleValueArgs NAME)
    set(multiValueArgs PORTABLE POSIX LINUX DEPENDS)
    cmake_parse_arguments(PARSE_ARGV 0 "ARG" "${options}" "${singleValueArgs}" "${multiValueArgs}")

    list(FILTER ARG_PORTABLE EXCLUDE REGEX "^$")
    list(FILTER ARG_POSIX EXCLUDE REGEX "^$")

    if(NOT ARG_NAME)
        message(FATAL_ERROR "uwin_sources with no name")
    endif()

    if(NOT ARG_PORTABLE AND ARG_POSIX AND ARG_LINUX)
        message(FATAL_ERROR "uwin_sources with no sources specified")
    endif()

    # we wrap files into OBJECT library to work around cmake quirks with generated source files not being properly exposed from subdirectory
    # https://gitlab.kitware.com/cmake/cmake/-/issues/18399
    add_library(${ARG_NAME} STATIC ${ARG_PORTABLE})
    if (UWIN_POSIX)
        target_sources(${ARG_NAME} PRIVATE ${ARG_POSIX})
    endif()
    if (UWIN_LINUX)
        target_sources(${ARG_NAME} PRIVATE ${ARG_LINUX})
    endif()

    target_link_libraries(${ARG_NAME} uwin-base)

    target_link_libraries(uwin ${ARG_NAME})

    target_link_libraries(${ARG_NAME} ${ARG_DEPENDS})

endfunction()


add_subdirectory(mem)
add_subdirectory(posix)
add_subdirectory(util)
add_subdirectory(win32)
add_subdirectory(xcute)

# TODO: this should definitely be integrated into the build system
target_sources(uwin PUBLIC /home/dcnick3/trash/homm3-switch/lifting/remill/cmake-build-debug-clang/tools/uwin-lift/lifted.o)

target_link_libraries(uwin fmt)

add_executable(uwin-main main.cpp)
target_link_libraries(uwin-main uwin)


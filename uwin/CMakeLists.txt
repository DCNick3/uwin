

set(UWIN_POSIX TRUE)
set(UWIN_LINUX TRUE)

# this is a meta-target to add includes and compiler definitions.
# probably want to add libraries here
add_library(uwin-base INTERFACE)

target_include_directories(uwin-base INTERFACE ${CMAKE_CURRENT_LIST_DIR} ${CMAKE_CURRENT_BINARY_DIR})

# TODO: make it into a normal package or add it to the source tree
target_include_directories(uwin-base INTERFACE /home/dcnick3/trash/homm3-switch/code/lifting/remill/include/)

add_library(uwin INTERFACE)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fmacro-prefix-map=${CMAKE_SOURCE_DIR}=src")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fmacro-prefix-map=${CMAKE_BINARY_DIR}=bin")

function(uwin_sources)
    set(options)
    set(singleValueArgs NAME)
    set(multiValueArgs PORTABLE POSIX LINUX DEPENDS)
    cmake_parse_arguments(PARSE_ARGV 0 "ARG" "${options}" "${singleValueArgs}" "${multiValueArgs}")

    list(FILTER ARG_PORTABLE EXCLUDE REGEX "^$")
    list(FILTER ARG_POSIX EXCLUDE REGEX "^$")

    if (NOT ARG_NAME)
        message(FATAL_ERROR "uwin_sources with no name")
    endif ()

    if (NOT ARG_PORTABLE AND ARG_POSIX AND ARG_LINUX)
        message(FATAL_ERROR "uwin_sources with no sources specified")
    endif ()

    # we wrap files into OBJECT library to work around cmake quirks with generated source files not being properly exposed from subdirectory
    # https://gitlab.kitware.com/cmake/cmake/-/issues/18399
    add_library(${ARG_NAME} STATIC ${ARG_PORTABLE})
    if (UWIN_POSIX)
        target_sources(${ARG_NAME} PRIVATE ${ARG_POSIX})
    endif ()
    if (UWIN_LINUX)
        target_sources(${ARG_NAME} PRIVATE ${ARG_LINUX})
    endif ()

    target_link_libraries(${ARG_NAME} PUBLIC uwin-base)

    target_link_libraries(uwin INTERFACE ${ARG_NAME})

    target_link_libraries(${ARG_NAME} PUBLIC ${ARG_DEPENDS})

endfunction()


add_subdirectory(mem)
add_subdirectory(posix)
add_subdirectory(util)
add_subdirectory(win32)
add_subdirectory(xcute)
add_subdirectory(log)
add_subdirectory(ht)
add_subdirectory(heap)
add_subdirectory(str)

target_link_libraries(uwin-base INTERFACE
        CONAN_PKG::fmt
        CONAN_PKG::di
        CONAN_PKG::icu
        kaitai-rt)

add_library(uwin-main STATIC main.cpp)
target_link_libraries(uwin-main uwin)

target_link_libraries(uwin INTERFACE uwin-base)

include(UWinEntry)

uwin_entry(NAME FloatTest EXE /home/dcnick3/trash/homm3-switch/code/test-exe/msvc/FloatTest/Release/FloatTest.exe)
uwin_entry(NAME SortTest EXE /home/dcnick3/trash/homm3-switch/code/test-exe/msvc/SortTest/Release/SortTest.exe)
uwin_entry(NAME OpenWatcomHelloWorld EXE /home/dcnick3/trash/homm3-switch/code/test-exe/owc/hello_world.exe)
uwin_entry(NAME OpenWatcomMatrix EXE /home/dcnick3/trash/homm3-switch/code/test-exe/owc-new/build/matrix.exe)


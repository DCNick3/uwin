cmake_minimum_required(VERSION 3.18)
project(uwin)

include("${CMAKE_BINARY_DIR}/conanbuildinfo.cmake")
conan_basic_setup(TARGETS)

set(CMAKE_CXX_STANDARD 20)

list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_LIST_DIR}/cmake)

find_package(GreatCMakeCookOff NO_MODULE PATHS ${CMAKE_CURRENT_LIST_DIR}/ext-deps/GreatCMakeCookOff REQUIRED)
initialize_cookoff()


include(CodeGen)
include(FindASan)

add_subdirectory(ext-deps/fmt)
add_subdirectory(ext-deps/gtest)
add_subdirectory(ext-deps/di)
add_subdirectory(ext-deps/icu)
add_subdirectory(kaitai-rt)

add_subdirectory(test)
add_subdirectory(uwin)
add_subdirectory(tools/winerr)



cmake_minimum_required(VERSION 3.18)
project(uwin)

set(CMAKE_CXX_STANDARD 14)

add_subdirectory(ext-deps/fmt)
add_subdirectory(ext-deps/gtest)

add_subdirectory(test)
add_subdirectory(uwin)



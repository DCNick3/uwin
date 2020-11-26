
set(CMAKE_CXX_STANDARD 20)

add_library(kaitai-rt STATIC
        kaitai/kaitaistream.cpp
        kaitai/kaitaispanin.cpp
        kaitai/kaitaistrin.cpp)

target_include_directories(kaitai-rt PUBLIC ${CMAKE_CURRENT_LIST_DIR})



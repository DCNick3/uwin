
add_executable(uwin-unit-tests
        AlignTests.cpp
        MemMapper.cpp
        TargetMemMgr.cpp
        MemRegion.cpp
        PagesRegion.cpp
        Win32Error.cpp
        HandleTable.cpp
        Heap.cpp
        Modules.cpp
        String.cpp
        remill_stubs.cpp)

target_link_libraries(uwin-unit-tests CONAN_PKG::gtest uwin)
add_test(NAME uwin-unit-tests COMMAND uwin-unit-tests)
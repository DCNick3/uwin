
add_executable(uwin-tests
        AlignTests.cpp
        MemMapper.cpp
        TargetMemMgr.cpp
        MemRegion.cpp
        PagesRegion.cpp
        Win32Error.cpp)

target_link_libraries(uwin-tests gtest_main uwin)
add_test(NAME uwin-tests COMMAND uwin-tests)
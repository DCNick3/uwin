include(OWCCompile)
include(UWinEntry)


owc_compile(OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/hello_world.exe" SOURCES "${CMAKE_CURRENT_LIST_DIR}/hello_world/hello_world.c")
owc_compile(OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/matrix.exe" SOURCES "${CMAKE_CURRENT_LIST_DIR}/matrix/matrix.c")

uwin_entry(NAME OpenWatcomHelloWorld EXE "${CMAKE_CURRENT_BINARY_DIR}/hello_world.exe")
uwin_entry(NAME OpenWatcomMatrix EXE "${CMAKE_CURRENT_BINARY_DIR}/matrix.exe")
uwin_entry(NAME MSVCFloatTest EXE "${CMAKE_CURRENT_LIST_DIR}/float_test/float_test.exe")
uwin_entry(NAME MSVCSortTest EXE "${CMAKE_CURRENT_LIST_DIR}/sort_test/sort_test.exe")
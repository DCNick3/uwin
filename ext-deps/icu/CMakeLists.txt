

include(ProcessorCount)
include(ExternalProject)

find_program(MAKE_PROGRAM make)

ProcessorCount(NUM_JOBS)

# TODO: cross-compilation
# (should I make a cmake build system for it?)

# UCONFIG_USE_LOCAL

set(HOST_CFLAGS "${CMAKE_C_FLAGS}")
set(HOST_CXXFLAGS "${CMAKE_CXX_FLAGS}")
set(HOST_CC "${CMAKE_C_COMPILER}")
set(HOST_CXX "${CMAKE_CXX_COMPILER}")
set(HOST_LDFLAGS "${CMAKE_MODULE_LINKER_FLAGS}")
set(HOST_CONFIGURE_FLAGS "")

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(HOST_CFLAGS "${HOST_CFLAGS} ${CMAKE_CXX_FLAGS_DEBUG}")
    set(HOST_CFLAGS "${HOST_CFLAGS} ${CMAKE_CXX_FLAGS_DEBUG}")
    #set(HOST_CONFIGURE_FLAGS "${HOST_CONFIGURE_FLAGS} --disable-release --enable-debug")
    list(APPEND HOST_CONFIGURE_FLAGS --disable-release --enable-debug)
elseif(CMAKE_BUILD_TYPE STREQUAL "ASan")
    set(HOST_CFLAGS "${HOST_CFLAGS} ${CMAKE_CXX_FLAGS_ASAN}")
    set(HOST_CFLAGS "${HOST_CFLAGS} ${CMAKE_CXX_FLAGS_ASAN}")
    set(HOST_LDFLAGS "${HOST_LDFLAGS} ${CMAKE_EXE_LINKER_FLAGS_ASAN}")
    list(APPEND HOST_CONFIGURE_FLAGS --disable-release --enable-debug)
else()
    list(APPEND HOST_CONFIGURE_FLAGS --disable-debug --enable-release)
    #set(HOST_CONFIGURE_FLAGS "${HOST_CONFIGURE_FLAGS} --disable-debug --enable-release")
endif()

message("HOST_CONFIGURE_FLAGS: ${HOST_CONFIGURE_FLAGS}")

set(HOST_ENV_CMAKE ${CMAKE_COMMAND} -E env
        CC=${HOST_CC}
        CXX=${HOST_CXX}
        "CFLAGS=-DUCONFIG_USE_LOCAL ${HOST_CFLAGS}"
        "CXXFLAGS=-DUCONFIG_USE_LOCAL ${HOST_CXXFLAGS}"
        "LDFLAGS=${HOST_LDFLAGS}"
        )

set(ICU_PREFIX "${CMAKE_CURRENT_BINARY_DIR}/icu-host")
ExternalProject_Add(icu-host
        PREFIX "${ICU_PREFIX}"
        URL https://github.com/unicode-org/icu/releases/download/release-69-1/icu4c-69_1-src.tgz
        URL_HASH SHA1=620a71c84428758376baa0fb81a581c3daa866ce
        PATCH_COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_LIST_DIR}/uconfig_local.h
            <SOURCE_DIR>/source/common/unicode/uconfig_local.h
        CONFIGURE_COMMAND ${HOST_ENV_CMAKE} <SOURCE_DIR>/source/configure
            --enable-static --disable-shared --disable-extras --disable-icuio
            --disable-tests --disable-samples --disable-draft --with-data-packaging=static
            --prefix=<INSTALL_DIR> ${HOST_CONFIGURE_FLAGS}
        BUILD_COMMAND ${HOST_ENV_CMAKE} ${MAKE_PROGRAM} -j ${NUM_JOBS}
        INSTALL_COMMAND ${HOST_ENV_CMAKE} ${MAKE_PROGRAM} install
)

find_package(Threads REQUIRED)

add_library(icu INTERFACE)
target_include_directories(icu INTERFACE "${ICU_PREFIX}/include")
target_link_directories(icu INTERFACE "${ICU_PREFIX}/lib")
target_link_libraries(icu INTERFACE icuuc icutu icui18n icudata Threads::Threads)


add_dependencies(icu icu-host)

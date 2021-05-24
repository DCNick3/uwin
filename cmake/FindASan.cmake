
set(SANITIZER ${CONAN_SETTINGS_COMPILER_SANITIZER})

if(SANITIZER)
    if(SANITIZER MATCHES "(Address)")
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=address -fno-omit-frame-pointer -fno-optimize-sibling-calls" )
    endif()
endif()
uwin_sources(NAME heap
        PORTABLE
            heap.cpp
            span.cpp
        DEPENDS win32 util)
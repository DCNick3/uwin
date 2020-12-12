uwin_sources(NAME heap
        PORTABLE
            heap.cpp
            span.cpp
            span_group.cpp
        DEPENDS win32 util mem-mgr)
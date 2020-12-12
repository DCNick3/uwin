uwin_sources(NAME mem-mgr
        PORTABLE pages_region.cpp pages_regions_container.cpp target_mem_mgr.cpp region_holder.cpp
        DEPENDS win32 log)
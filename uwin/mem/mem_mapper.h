//
// Created by dcnick3 on 10/25/20.
//

#pragma once

#include "mem/base_mem_mapper.h"


namespace uwin::mem {
    std::shared_ptr<base_mem_mapper> create_host_mem_mapper();
}

/*
#ifdef UWIN_POSIX

#include "mem/posix/posix_mem_mapper.h"
namespace uwin {
    namespace mem {
        using mem_mapper = posix_mem_mapper;
    }
}

#else
#error "No backend for memory mapping for this platform"
#endif*/

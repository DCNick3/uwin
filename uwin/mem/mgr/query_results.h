//
// Created by dcnick3 on 4/19/21.
//

#pragma once

namespace uwin::mem::mgr::query_results {
    struct free {
        tmem_region region;
    };
    struct reserved {
        tmem_region region;
        tmem_region base_region;
    };
    struct committed {
        tmem_region region;
        tmem_region base_region;
        tprot protection;
    };
}
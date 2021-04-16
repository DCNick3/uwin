//
// Created by dcnick3 on 4/16/21.
//

#pragma once

#include "mem/tptr.h"

namespace uwin::win32::types {

    struct module_tag;

    typedef uwin::mem::tcptr<module_tag> hmodule;
}
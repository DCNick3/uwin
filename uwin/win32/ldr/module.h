//
// Created by dcnick3 on 11/15/20.
//

#pragma once

#include "mem/tptr.h"
#include "mem/mem_region.h"
#include "win32/ldr/linkable.h"
#include "win32/ldr/microsoft_pe.h"

#include <string>
#include <utility>

namespace uwin::win32::ldr {
    class module : public linkable {
        const std::string _module_name;
        const mem::tmem_region _image_region;
        const mem::tcaddr _entry_point_address;

    public:
        module(const mem::tmem_region &image_region, std::string module_name,
               const mem::tcaddr &entry_point_address) : _image_region(image_region), _module_name(std::move(module_name)),
                                                         _entry_point_address(entry_point_address)
        {}

        [[nodiscard]] mem::tcaddr try_resolve(const std::string &symbol_name) const override;

        [[nodiscard]] inline const std::string& name() const { return _module_name; }
        [[nodiscard]] inline mem::tcaddr entrypoint() const { return _entry_point_address; }

        module(const module&) = delete;
        void operator=(const module&) = delete;
    };
}
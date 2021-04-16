//
// Created by dcnick3 on 11/14/20.
//

#pragma once

#include "ctx/process.h"
#include "win32/ldr/microsoft_pe.h"
#include "win32/ldr/target_module.h"
#include "win32/ldr/module_table.h"
#include "util/span.h"

#include <span>
#include <sstream>

namespace uwin::win32::ldr {
    template<typename T = std::uint8_t>
    class rva {
    public:
        std::uint32_t value;
    };

    static_assert(sizeof(rva<>) == 4, "unexpected rva size");
    static_assert(alignof(rva<>) == 4, "unexpected rva alignment");
    static_assert(std::is_trivial_v<rva<>> && std::is_standard_layout_v<rva<>>,
                  "expected rva to be pod");

    class module_loader {
        module_table& _module_table;
        mem::mgr::target_mem_mgr &_mem_mgr;

        // kaitai-generated code is quite inefficient... But oh well
        kaitai::kstream _kaitai_stream;
        microsoft_pe_t _pe_image;

        std::span<const std::uint8_t> _image;

        mem::taddr _image_base;
        mem::taddr _desired_image_base = 0;
        mem::tmem_region _image_region;
        mem::hmem_region _himage_region;

        std::string _module_name;

        module_loader(ldr::module_table& module_table,
                      mem::mgr::target_mem_mgr &mem_mgr,
                      mem::taddr image_base,
                      std::span<const std::uint8_t> image,
                      std::string module_name)
                :
                _module_table(module_table),
                _mem_mgr(mem_mgr),
                _kaitai_stream(image),
                _pe_image(&_kaitai_stream),
                _image_base(image_base),
                _image(image),
                _module_name(std::move(module_name)),
                _image_region(0, 0),
                _himage_region(nullptr, 0) {
            _pe_image.pe();
        }

        template<typename T = std::uint8_t>
        std::span<T> get_loaded_directory_data(microsoft_pe_t::data_dir_t const& dir);

        template<typename T>
        inline mem::tptr<T> resolve(rva<T> rva) {
            auto res = _image_base + rva.value;
            return {res.value()};
        }
        template<typename T>
        inline T* ptr(rva<T> rva) {
            assert(util::is_aligned(rva.value, alignof(T)));
            return _mem_mgr.ptr(resolve(rva));
        }

        template<typename T>
        inline T& deref(rva<T> rva) {
            return *ptr(rva);
        }

        inline std::string_view str(rva<char> rva) {
            return _mem_mgr.str(resolve(rva));
        }

        void apply_relocs();
        void resolve_imports();

        target_module& load_impl();

    public:

        static inline target_module& load(ldr::module_table& module_table,
                                          mem::mgr::target_mem_mgr &mem_mgr,
                                          mem::taddr image_base,
                                          std::string name,
                                          std::span<std::uint8_t> const &image) {
            module_loader ldr(module_table, mem_mgr, image_base, image, std::move(name));
#pragma clang diagnostic push
#pragma ide diagnostic ignored "EndlessLoop"
            return ldr.load_impl();
#pragma clang diagnostic pop
        }
    };
}




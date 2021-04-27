//
// Created by dcnick3 on 11/14/20.
//


#include <kaitai/kaitaistream.h>

#include "win32/ldr/module_loader.h"
#include "win32/ldr/except.h"
#include "mem/mgr/consts.h"
#include "ctx/dll.h"
#include "util/enumu.h"
#include "util/str.h"
#include "util/align.h"
#include "log/log.h"

#include <cassert>
#include <iostream>

namespace uwin::win32::ldr {

    namespace executable_characteristics {
        enum {
            IMAGE_FILE_RELOCS_STRIPPED = 0x0001,
            IMAGE_FILE_EXECUTABLE_IMAGE = 0x0002,
        };
    }

    enum class section_characteristics : std::uint32_t {
        MEM_EXECUTE = 0x20000000,
        MEM_READ = 0x40000000,
        MEM_WRITE = 0x80000000,
    };
}

template<>
struct uwin::EnableBitMaskOperators<uwin::win32::ldr::section_characteristics> {
    static const bool enable = true;
};

namespace uwin::win32::ldr {

    static mem::mgr::tprot get_section_prot(section_characteristics characteristics) {
        unsigned r = characteristics % section_characteristics::MEM_READ;
        unsigned w = characteristics % section_characteristics::MEM_WRITE;
        unsigned x = characteristics % section_characteristics::MEM_EXECUTE;
        switch ((r << 2U)| (w << 1U) | (x << 0U) ) {
            case 0b100: return mem::mgr::tprot::r;
            case 0b010: return mem::mgr::tprot::w;
            case 0b001: return mem::mgr::tprot::x;
            case 0b110: return mem::mgr::tprot::rw;
            case 0b101: return mem::mgr::tprot::rx;
            case 0b011: return mem::mgr::tprot::wx;
            case 0b111: return mem::mgr::tprot::rwx;
            default: std::terminate();
        }
    }

    target_module &ldr::module_loader::load_impl() {
        log::debug("loading {}", _module_name);

        auto pe = _pe_image.pe();

        if (pe->coff_hdr()->machine() != microsoft_pe_t::coff_header_t::machine_type_t::MACHINE_TYPE_I386)
            throw loader_exception("Unsupported architecture");
        if (pe->optional_hdr()->std()->format() != microsoft_pe_t::pe_format_t::PE_FORMAT_PE32)
            throw loader_exception("Unsupported executable format");

        auto section_alignment = pe->optional_hdr()->windows()->section_alignment();
        _desired_image_base = mem::taddr(pe->optional_hdr()->windows()->image_base_32());

        if (_image_base != _desired_image_base &&
            (pe->coff_hdr()->characteristics() & executable_characteristics::IMAGE_FILE_RELOCS_STRIPPED) != 0)
            throw loader_exception("Executable needs relocation, though it does not have reloc info");
        if (section_alignment != mem::mgr::consts::page_size)
            throw loader_exception("Not sure what to do with file whose section_alignment is not the same as page size");

        // stage 1: map memory for sections & load their data
        // stage 2: apply relocs
        // stage 3: resolve imports
        // stage 4: apply needed memory protection for sections
        // stage 5: construct a `module` object
        // stage 6: register it in loader context object
        // ???
        // PROFIT

        _image_region = _mem_mgr.reserve_fixed(
                    mem::tmem_region(_image_base, pe->optional_hdr()->windows()->size_of_image())
                );
        _himage_region = _mem_mgr.ptr(_image_region);

        // map memory for sections & load their data
        for (auto const& sec : *pe->sections()) {
            auto alloc_size = util::align_up(sec->virtual_size(), section_alignment);

            if (alloc_size == 0)
                continue;

            auto virtual_region = mem::tmem_region(_image_base + sec->virtual_address(), alloc_size);
            _mem_mgr.commit(virtual_region, mem::mgr::tprot::rw);

            auto host_region = _mem_mgr.ptr(virtual_region);

            auto data = _image.subspan(sec->pointer_to_raw_data(), sec->size_of_raw_data());

            assert(data.size() <= host_region.size());

            memset(host_region.begin(), 0, host_region.size());
            memcpy(host_region.begin(), data.data(), data.size());
        }

        apply_relocs();
        resolve_imports();

        // apply needed memory protection for sections
        for (auto const& sec : *pe->sections()) {
            auto alloc_size = util::align_up(sec->virtual_size(), section_alignment);

            auto virtual_region = mem::tmem_region(_image_base + sec->virtual_address(), alloc_size);

            auto prot = get_section_prot(static_cast<section_characteristics>(sec->characteristics()));
            _mem_mgr.reprotect(virtual_region, prot);
        }

        // TODO: implement auto conversion of tcaddr to taddr
        auto entrypoint = mem::tcaddr((_image_base + pe->optional_hdr()->std()->address_of_entry_point()).value());

        return _module_table.emplace_target_module(_image_region, _module_name, entrypoint);
    }

    template<typename T>
    std::span<T> module_loader::get_loaded_directory_data(const microsoft_pe_t::data_dir_t &dir) {
        // OpenWatcom quirk: import directory size not divisible by entry size. Not sure what is there,
        //   ignoring leftover data for now by rounding it down
        mem::tmem_region rg(_image_base + dir.virtual_address(), util::align_down(dir.size(), sizeof(T)));
        assert(rg.is_contained(_image_region));
        return _mem_mgr.ptr(rg).as_span<T>();
    }

    namespace i368_reloc {
        enum {
            absolute = 0x0,
            dir32 = 0x6,
            dir32nb = 0x7,
            section = 0xa,
            secrel = 0xb,
            token = 0xc,
            secrel7 = 0xd,
            rel32 = 0x14,

        };
    }

    void module_loader::apply_relocs() {
        if (_desired_image_base != _image_base) {
            throw loader_exception("relocations are not implemented");
        }
    }

    struct [[gnu::packed]] hint_name {
        std::uint16_t hint;
        char name[]; // Is it safe?)
    };

    struct [[gnu::packed]] lookup_table {
        std::uint32_t value;

        [[nodiscard]] bool is_ordinal() const {
            return (value & 0x80000000) != 0;
        }
        [[nodiscard]] std::uint16_t get_ordinal() const {
            return value & 0xffffU;
        }
        [[nodiscard]] rva<hint_name> get_hint_name() const {
            return {value & 0x7fffffffU};
        }
    };

    struct [[gnu::packed]] import_directory_entry {
        rva<lookup_table> plookup_table;
        std::uint32_t timestamp;
        std::uint32_t forwarder_chain;
        rva<char> name;
        rva<mem::tcaddrpod> address_table;
    };

    void module_loader::resolve_imports() {
        auto table = get_loaded_directory_data<import_directory_entry>(
                *_pe_image.pe()->optional_hdr()->data_dirs()->import_table()
                );
        for (auto const& entry : table) {
            if (entry.name.value == 0)
                break; // end-of-table

            // TODO: should we use the same normalization algorithm as GetModuleHandle does?
            auto dll_name = util::ascii_to_upper(str(entry.name));

            auto& module = _module_table.get_module(dll_name);

            log::debug("importing {}", dll_name);

            std::vector<std::string> missing_imports;

            auto plookup_table = ptr(entry.plookup_table);
            for (int i = 0; plookup_table[i].value != 0; i++) {
                auto& lookup = plookup_table[i];
                if (lookup.is_ordinal())
                    throw loader_exception("Import by ordinal is not implemented.");

                std::string symbol_name(deref(lookup.get_hint_name()).name);

                log::debug("  {}", symbol_name);

                auto res = module.try_resolve(symbol_name);

                if (res == 0)
                    missing_imports.emplace_back(symbol_name);

                ptr(entry.address_table)[i] = mem::tcaddrpod::from_tptr(res);
            }

            if (!missing_imports.empty()) {
                std::string r;
                for (auto const& v : missing_imports) {
                    r += fmt::format("{}\n", v);
                }
                if (!r.empty())
                    r.erase(r.size() - 1);

                throw loader_exception(fmt::format("Missing imports from {}:\n{}", dll_name, r));
            }

        }
    }
}

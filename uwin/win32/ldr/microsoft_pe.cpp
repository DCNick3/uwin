// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#include "microsoft_pe.h"
#include "kaitai/exceptions.h"
namespace uwin {
    namespace win32 {
        namespace ldr {

            microsoft_pe_t::microsoft_pe_t(kaitai::kstream* p__io, kaitai::kstruct* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = this;
                m_mz = nullptr;
                m_pe = nullptr;
                f_pe = false;
                _read();
            }

            void microsoft_pe_t::_read() {
                m_mz = std::unique_ptr<mz_placeholder_t>(new mz_placeholder_t(m__io, this, m__root));
            }

            microsoft_pe_t::~microsoft_pe_t() {
                _clean_up();
            }

            void microsoft_pe_t::_clean_up() {
                if (f_pe) {
                }
            }

            microsoft_pe_t::certificate_entry_t::certificate_entry_t(kaitai::kstream* p__io, microsoft_pe_t::certificate_table_t* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = p__root;
                _read();
            }

            void microsoft_pe_t::certificate_entry_t::_read() {
                m_length = m__io->read_u4le();
                m_revision = static_cast<microsoft_pe_t::certificate_entry_t::certificate_revision_t>(m__io->read_u2le());
                m_certificate_type = static_cast<microsoft_pe_t::certificate_entry_t::certificate_type_t>(m__io->read_u2le());
                m_certificate_bytes = m__io->read_bytes((length() - 8));
            }

            microsoft_pe_t::certificate_entry_t::~certificate_entry_t() {
                _clean_up();
            }

            void microsoft_pe_t::certificate_entry_t::_clean_up() {
            }

            microsoft_pe_t::optional_header_windows_t::optional_header_windows_t(kaitai::kstream* p__io, microsoft_pe_t::optional_header_t* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = p__root;
                _read();
            }

            void microsoft_pe_t::optional_header_windows_t::_read() {
                n_image_base_32 = true;
                if (_parent()->std()->format() == microsoft_pe_t::PE_FORMAT_PE32) {
                    n_image_base_32 = false;
                    m_image_base_32 = m__io->read_u4le();
                }
                n_image_base_64 = true;
                if (_parent()->std()->format() == microsoft_pe_t::PE_FORMAT_PE32_PLUS) {
                    n_image_base_64 = false;
                    m_image_base_64 = m__io->read_u8le();
                }
                m_section_alignment = m__io->read_u4le();
                m_file_alignment = m__io->read_u4le();
                m_major_operating_system_version = m__io->read_u2le();
                m_minor_operating_system_version = m__io->read_u2le();
                m_major_image_version = m__io->read_u2le();
                m_minor_image_version = m__io->read_u2le();
                m_major_subsystem_version = m__io->read_u2le();
                m_minor_subsystem_version = m__io->read_u2le();
                m_win32_version_value = m__io->read_u4le();
                m_size_of_image = m__io->read_u4le();
                m_size_of_headers = m__io->read_u4le();
                m_check_sum = m__io->read_u4le();
                m_subsystem = static_cast<microsoft_pe_t::optional_header_windows_t::subsystem_enum_t>(m__io->read_u2le());
                m_dll_characteristics = m__io->read_u2le();
                n_size_of_stack_reserve_32 = true;
                if (_parent()->std()->format() == microsoft_pe_t::PE_FORMAT_PE32) {
                    n_size_of_stack_reserve_32 = false;
                    m_size_of_stack_reserve_32 = m__io->read_u4le();
                }
                n_size_of_stack_reserve_64 = true;
                if (_parent()->std()->format() == microsoft_pe_t::PE_FORMAT_PE32_PLUS) {
                    n_size_of_stack_reserve_64 = false;
                    m_size_of_stack_reserve_64 = m__io->read_u8le();
                }
                n_size_of_stack_commit_32 = true;
                if (_parent()->std()->format() == microsoft_pe_t::PE_FORMAT_PE32) {
                    n_size_of_stack_commit_32 = false;
                    m_size_of_stack_commit_32 = m__io->read_u4le();
                }
                n_size_of_stack_commit_64 = true;
                if (_parent()->std()->format() == microsoft_pe_t::PE_FORMAT_PE32_PLUS) {
                    n_size_of_stack_commit_64 = false;
                    m_size_of_stack_commit_64 = m__io->read_u8le();
                }
                n_size_of_heap_reserve_32 = true;
                if (_parent()->std()->format() == microsoft_pe_t::PE_FORMAT_PE32) {
                    n_size_of_heap_reserve_32 = false;
                    m_size_of_heap_reserve_32 = m__io->read_u4le();
                }
                n_size_of_heap_reserve_64 = true;
                if (_parent()->std()->format() == microsoft_pe_t::PE_FORMAT_PE32_PLUS) {
                    n_size_of_heap_reserve_64 = false;
                    m_size_of_heap_reserve_64 = m__io->read_u8le();
                }
                n_size_of_heap_commit_32 = true;
                if (_parent()->std()->format() == microsoft_pe_t::PE_FORMAT_PE32) {
                    n_size_of_heap_commit_32 = false;
                    m_size_of_heap_commit_32 = m__io->read_u4le();
                }
                n_size_of_heap_commit_64 = true;
                if (_parent()->std()->format() == microsoft_pe_t::PE_FORMAT_PE32_PLUS) {
                    n_size_of_heap_commit_64 = false;
                    m_size_of_heap_commit_64 = m__io->read_u8le();
                }
                m_loader_flags = m__io->read_u4le();
                m_number_of_rva_and_sizes = m__io->read_u4le();
            }

            microsoft_pe_t::optional_header_windows_t::~optional_header_windows_t() {
                _clean_up();
            }

            void microsoft_pe_t::optional_header_windows_t::_clean_up() {
                if (!n_image_base_32) {
                }
                if (!n_image_base_64) {
                }
                if (!n_size_of_stack_reserve_32) {
                }
                if (!n_size_of_stack_reserve_64) {
                }
                if (!n_size_of_stack_commit_32) {
                }
                if (!n_size_of_stack_commit_64) {
                }
                if (!n_size_of_heap_reserve_32) {
                }
                if (!n_size_of_heap_reserve_64) {
                }
                if (!n_size_of_heap_commit_32) {
                }
                if (!n_size_of_heap_commit_64) {
                }
            }

            microsoft_pe_t::optional_header_data_dirs_t::optional_header_data_dirs_t(kaitai::kstream* p__io, microsoft_pe_t::optional_header_t* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = p__root;
                m_export_table = nullptr;
                m_import_table = nullptr;
                m_resource_table = nullptr;
                m_exception_table = nullptr;
                m_certificate_table = nullptr;
                m_base_relocation_table = nullptr;
                m_debug = nullptr;
                m_architecture = nullptr;
                m_global_ptr = nullptr;
                m_tls_table = nullptr;
                m_load_config_table = nullptr;
                m_bound_import = nullptr;
                m_iat = nullptr;
                m_delay_import_descriptor = nullptr;
                m_clr_runtime_header = nullptr;
                _read();
            }

            void microsoft_pe_t::optional_header_data_dirs_t::_read() {
                m_export_table = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_import_table = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_resource_table = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_exception_table = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_certificate_table = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_base_relocation_table = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_debug = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_architecture = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_global_ptr = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_tls_table = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_load_config_table = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_bound_import = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_iat = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_delay_import_descriptor = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
                m_clr_runtime_header = std::unique_ptr<data_dir_t>(new data_dir_t(m__io, this, m__root));
            }

            microsoft_pe_t::optional_header_data_dirs_t::~optional_header_data_dirs_t() {
                _clean_up();
            }

            void microsoft_pe_t::optional_header_data_dirs_t::_clean_up() {
            }

            microsoft_pe_t::data_dir_t::data_dir_t(kaitai::kstream* p__io, microsoft_pe_t::optional_header_data_dirs_t* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = p__root;
                _read();
            }

            void microsoft_pe_t::data_dir_t::_read() {
                m_virtual_address = m__io->read_u4le();
                m_size = m__io->read_u4le();
            }

            microsoft_pe_t::data_dir_t::~data_dir_t() {
                _clean_up();
            }

            void microsoft_pe_t::data_dir_t::_clean_up() {
            }

            microsoft_pe_t::coff_symbol_t::coff_symbol_t(kaitai::kstream* p__io, microsoft_pe_t::coff_header_t* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = p__root;
                m_name_annoying = nullptr;
                m__io__raw_name_annoying = nullptr;
                f_section = false;
                f_data = false;
                _read();
            }

            void microsoft_pe_t::coff_symbol_t::_read() {
                m__raw_name_annoying = m__io->read_bytes(8);
                m__io__raw_name_annoying = std::unique_ptr<kaitai::kstream>(new kaitai::kstream(m__raw_name_annoying));
                m_name_annoying = std::unique_ptr<annoyingstring_t>(new annoyingstring_t(m__io__raw_name_annoying.get(), this, m__root));
                m_value = m__io->read_u4le();
                m_section_number = m__io->read_u2le();
                m_type = m__io->read_u2le();
                m_storage_class = m__io->read_u1();
                m_number_of_aux_symbols = m__io->read_u1();
            }

            microsoft_pe_t::coff_symbol_t::~coff_symbol_t() {
                _clean_up();
            }

            void microsoft_pe_t::coff_symbol_t::_clean_up() {
                if (f_data) {
                }
            }

            microsoft_pe_t::section_t* microsoft_pe_t::coff_symbol_t::section() {
                if (f_section)
                    return m_section;
                m_section = _root()->pe()->sections()->at((section_number() - 1)).get();
                f_section = true;
                return m_section;
            }

            std::string microsoft_pe_t::coff_symbol_t::data() {
                if (f_data)
                    return m_data;
                std::streampos _pos = m__io->pos();
                m__io->seek((section()->pointer_to_raw_data() + value()));
                m_data = m__io->read_bytes(1);
                m__io->seek(_pos);
                f_data = true;
                return m_data;
            }

            microsoft_pe_t::pe_header_t::pe_header_t(kaitai::kstream* p__io, microsoft_pe_t* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = p__root;
                m_coff_hdr = nullptr;
                m_optional_hdr = nullptr;
                m__io__raw_optional_hdr = nullptr;
                m_sections = nullptr;
                m_certificate_table = nullptr;
                m__io__raw_certificate_table = nullptr;
                f_certificate_table = false;
                _read();
            }

            void microsoft_pe_t::pe_header_t::_read() {
                m_pe_signature = m__io->read_bytes(4);
                if (!(pe_signature() == std::string("\x50\x45\x00\x00", 4))) {
                    throw kaitai::validation_not_equal_error<std::string>(std::string("\x50\x45\x00\x00", 4), pe_signature(), _io(), std::string("/types/pe_header/seq/0"));
                }
                m_coff_hdr = std::unique_ptr<coff_header_t>(new coff_header_t(m__io, this, m__root));
                m__raw_optional_hdr = m__io->read_bytes(coff_hdr()->size_of_optional_header());
                m__io__raw_optional_hdr = std::unique_ptr<kaitai::kstream>(new kaitai::kstream(m__raw_optional_hdr));
                m_optional_hdr = std::unique_ptr<optional_header_t>(new optional_header_t(m__io__raw_optional_hdr.get(), this, m__root));
                int l_sections = coff_hdr()->number_of_sections();
                m_sections = std::unique_ptr<std::vector<std::unique_ptr<section_t>>>(new std::vector<std::unique_ptr<section_t>>());
                m_sections->reserve(l_sections);
                for (int i = 0; i < l_sections; i++) {
                    m_sections->push_back(std::move(std::unique_ptr<section_t>(new section_t(m__io, this, m__root))));
                }
            }

            microsoft_pe_t::pe_header_t::~pe_header_t() {
                _clean_up();
            }

            void microsoft_pe_t::pe_header_t::_clean_up() {
                if (f_certificate_table && !n_certificate_table) {
                }
            }

            microsoft_pe_t::certificate_table_t* microsoft_pe_t::pe_header_t::certificate_table() {
                if (f_certificate_table)
                    return m_certificate_table.get();
                n_certificate_table = true;
                if (optional_hdr()->data_dirs()->certificate_table()->virtual_address() != 0) {
                    n_certificate_table = false;
                    std::streampos _pos = m__io->pos();
                    m__io->seek(optional_hdr()->data_dirs()->certificate_table()->virtual_address());
                    m__raw_certificate_table = m__io->read_bytes(optional_hdr()->data_dirs()->certificate_table()->size());
                    m__io__raw_certificate_table = std::unique_ptr<kaitai::kstream>(new kaitai::kstream(m__raw_certificate_table));
                    m_certificate_table = std::unique_ptr<certificate_table_t>(new certificate_table_t(m__io__raw_certificate_table.get(), this, m__root));
                    m__io->seek(_pos);
                    f_certificate_table = true;
                }
                return m_certificate_table.get();
            }

            microsoft_pe_t::optional_header_t::optional_header_t(kaitai::kstream* p__io, microsoft_pe_t::pe_header_t* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = p__root;
                m_std = nullptr;
                m_windows = nullptr;
                m_data_dirs = nullptr;
                _read();
            }

            void microsoft_pe_t::optional_header_t::_read() {
                m_std = std::unique_ptr<optional_header_std_t>(new optional_header_std_t(m__io, this, m__root));
                m_windows = std::unique_ptr<optional_header_windows_t>(new optional_header_windows_t(m__io, this, m__root));
                m_data_dirs = std::unique_ptr<optional_header_data_dirs_t>(new optional_header_data_dirs_t(m__io, this, m__root));
            }

            microsoft_pe_t::optional_header_t::~optional_header_t() {
                _clean_up();
            }

            void microsoft_pe_t::optional_header_t::_clean_up() {
            }

            microsoft_pe_t::section_t::section_t(kaitai::kstream* p__io, microsoft_pe_t::pe_header_t* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = p__root;
                f_body = false;
                _read();
            }

            void microsoft_pe_t::section_t::_read() {
                m_name = kaitai::kstream::bytes_to_str(kaitai::kstream::bytes_strip_right(m__io->read_bytes(8), 0), std::string("UTF-8"));
                m_virtual_size = m__io->read_u4le();
                m_virtual_address = m__io->read_u4le();
                m_size_of_raw_data = m__io->read_u4le();
                m_pointer_to_raw_data = m__io->read_u4le();
                m_pointer_to_relocations = m__io->read_u4le();
                m_pointer_to_linenumbers = m__io->read_u4le();
                m_number_of_relocations = m__io->read_u2le();
                m_number_of_linenumbers = m__io->read_u2le();
                m_characteristics = m__io->read_u4le();
            }

            microsoft_pe_t::section_t::~section_t() {
                _clean_up();
            }

            void microsoft_pe_t::section_t::_clean_up() {
                if (f_body) {
                }
            }

            std::string microsoft_pe_t::section_t::body() {
                if (f_body)
                    return m_body;
                std::streampos _pos = m__io->pos();
                m__io->seek(pointer_to_raw_data());
                m_body = m__io->read_bytes(size_of_raw_data());
                m__io->seek(_pos);
                f_body = true;
                return m_body;
            }

            microsoft_pe_t::certificate_table_t::certificate_table_t(kaitai::kstream* p__io, microsoft_pe_t::pe_header_t* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = p__root;
                m_items = nullptr;
                _read();
            }

            void microsoft_pe_t::certificate_table_t::_read() {
                m_items = std::unique_ptr<std::vector<std::unique_ptr<certificate_entry_t>>>(new std::vector<std::unique_ptr<certificate_entry_t>>());
                {
                    int i = 0;
                    while (!m__io->is_eof()) {
                        m_items->push_back(std::move(std::unique_ptr<certificate_entry_t>(new certificate_entry_t(m__io, this, m__root))));
                        i++;
                    }
                }
            }

            microsoft_pe_t::certificate_table_t::~certificate_table_t() {
                _clean_up();
            }

            void microsoft_pe_t::certificate_table_t::_clean_up() {
            }

            microsoft_pe_t::mz_placeholder_t::mz_placeholder_t(kaitai::kstream* p__io, microsoft_pe_t* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = p__root;
                _read();
            }

            void microsoft_pe_t::mz_placeholder_t::_read() {
                m_magic = m__io->read_bytes(2);
                if (!(magic() == std::string("\x4D\x5A", 2))) {
                    throw kaitai::validation_not_equal_error<std::string>(std::string("\x4D\x5A", 2), magic(), _io(), std::string("/types/mz_placeholder/seq/0"));
                }
                m_data1 = m__io->read_bytes(58);
                m_ofs_pe = m__io->read_u4le();
            }

            microsoft_pe_t::mz_placeholder_t::~mz_placeholder_t() {
                _clean_up();
            }

            void microsoft_pe_t::mz_placeholder_t::_clean_up() {
            }

            microsoft_pe_t::optional_header_std_t::optional_header_std_t(kaitai::kstream* p__io, microsoft_pe_t::optional_header_t* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = p__root;
                _read();
            }

            void microsoft_pe_t::optional_header_std_t::_read() {
                m_format = static_cast<microsoft_pe_t::pe_format_t>(m__io->read_u2le());
                m_major_linker_version = m__io->read_u1();
                m_minor_linker_version = m__io->read_u1();
                m_size_of_code = m__io->read_u4le();
                m_size_of_initialized_data = m__io->read_u4le();
                m_size_of_uninitialized_data = m__io->read_u4le();
                m_address_of_entry_point = m__io->read_u4le();
                m_base_of_code = m__io->read_u4le();
                n_base_of_data = true;
                if (format() == microsoft_pe_t::PE_FORMAT_PE32) {
                    n_base_of_data = false;
                    m_base_of_data = m__io->read_u4le();
                }
            }

            microsoft_pe_t::optional_header_std_t::~optional_header_std_t() {
                _clean_up();
            }

            void microsoft_pe_t::optional_header_std_t::_clean_up() {
                if (!n_base_of_data) {
                }
            }

            microsoft_pe_t::coff_header_t::coff_header_t(kaitai::kstream* p__io, microsoft_pe_t::pe_header_t* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = p__root;
                m_symbol_table = nullptr;
                f_symbol_table_size = false;
                f_symbol_name_table_offset = false;
                f_symbol_name_table_size = false;
                f_symbol_table = false;
                _read();
            }

            void microsoft_pe_t::coff_header_t::_read() {
                m_machine = static_cast<microsoft_pe_t::coff_header_t::machine_type_t>(m__io->read_u2le());
                m_number_of_sections = m__io->read_u2le();
                m_time_date_stamp = m__io->read_u4le();
                m_pointer_to_symbol_table = m__io->read_u4le();
                m_number_of_symbols = m__io->read_u4le();
                m_size_of_optional_header = m__io->read_u2le();
                m_characteristics = m__io->read_u2le();
            }

            microsoft_pe_t::coff_header_t::~coff_header_t() {
                _clean_up();
            }

            void microsoft_pe_t::coff_header_t::_clean_up() {
                if (f_symbol_name_table_size) {
                }
                if (f_symbol_table) {
                }
            }

            int32_t microsoft_pe_t::coff_header_t::symbol_table_size() {
                if (f_symbol_table_size)
                    return m_symbol_table_size;
                m_symbol_table_size = (number_of_symbols() * 18);
                f_symbol_table_size = true;
                return m_symbol_table_size;
            }

            int32_t microsoft_pe_t::coff_header_t::symbol_name_table_offset() {
                if (f_symbol_name_table_offset)
                    return m_symbol_name_table_offset;
                m_symbol_name_table_offset = (pointer_to_symbol_table() + symbol_table_size());
                f_symbol_name_table_offset = true;
                return m_symbol_name_table_offset;
            }

            uint32_t microsoft_pe_t::coff_header_t::symbol_name_table_size() {
                if (f_symbol_name_table_size)
                    return m_symbol_name_table_size;
                std::streampos _pos = m__io->pos();
                m__io->seek(symbol_name_table_offset());
                m_symbol_name_table_size = m__io->read_u4le();
                m__io->seek(_pos);
                f_symbol_name_table_size = true;
                return m_symbol_name_table_size;
            }

            std::vector<std::unique_ptr<microsoft_pe_t::coff_symbol_t>>* microsoft_pe_t::coff_header_t::symbol_table() {
                if (f_symbol_table)
                    return m_symbol_table.get();
                std::streampos _pos = m__io->pos();
                m__io->seek(pointer_to_symbol_table());
                int l_symbol_table = number_of_symbols();
                m_symbol_table = std::unique_ptr<std::vector<std::unique_ptr<coff_symbol_t>>>(new std::vector<std::unique_ptr<coff_symbol_t>>());
                m_symbol_table->reserve(l_symbol_table);
                for (int i = 0; i < l_symbol_table; i++) {
                    m_symbol_table->push_back(std::move(std::unique_ptr<coff_symbol_t>(new coff_symbol_t(m__io, this, m__root))));
                }
                m__io->seek(_pos);
                f_symbol_table = true;
                return m_symbol_table.get();
            }

            microsoft_pe_t::annoyingstring_t::annoyingstring_t(kaitai::kstream* p__io, microsoft_pe_t::coff_symbol_t* p__parent, microsoft_pe_t* p__root) : kaitai::kstruct(p__io) {
                m__parent = p__parent;
                m__root = p__root;
                f_name_from_offset = false;
                f_name_offset = false;
                f_name = false;
                f_name_zeroes = false;
                f_name_from_short = false;
                _read();
            }

            void microsoft_pe_t::annoyingstring_t::_read() {
            }

            microsoft_pe_t::annoyingstring_t::~annoyingstring_t() {
                _clean_up();
            }

            void microsoft_pe_t::annoyingstring_t::_clean_up() {
                if (f_name_from_offset && !n_name_from_offset) {
                }
                if (f_name_offset) {
                }
                if (f_name_zeroes) {
                }
                if (f_name_from_short && !n_name_from_short) {
                }
            }

            std::string microsoft_pe_t::annoyingstring_t::name_from_offset() {
                if (f_name_from_offset)
                    return m_name_from_offset;
                n_name_from_offset = true;
                if (name_zeroes() == 0) {
                    n_name_from_offset = false;
                    kaitai::kstream *io = _root()->_io();
                    std::streampos _pos = io->pos();
                    io->seek(((name_zeroes() == 0) ? ((_parent()->_parent()->symbol_name_table_offset() + name_offset())) : (0)));
                    m_name_from_offset = kaitai::kstream::bytes_to_str(io->read_bytes_term(0, false, true, false), std::string("ascii"));
                    io->seek(_pos);
                    f_name_from_offset = true;
                }
                return m_name_from_offset;
            }

            uint32_t microsoft_pe_t::annoyingstring_t::name_offset() {
                if (f_name_offset)
                    return m_name_offset;
                std::streampos _pos = m__io->pos();
                m__io->seek(4);
                m_name_offset = m__io->read_u4le();
                m__io->seek(_pos);
                f_name_offset = true;
                return m_name_offset;
            }

            std::string microsoft_pe_t::annoyingstring_t::name() {
                if (f_name)
                    return m_name;
                m_name = ((name_zeroes() == 0) ? (name_from_offset()) : (name_from_short()));
                f_name = true;
                return m_name;
            }

            uint32_t microsoft_pe_t::annoyingstring_t::name_zeroes() {
                if (f_name_zeroes)
                    return m_name_zeroes;
                std::streampos _pos = m__io->pos();
                m__io->seek(0);
                m_name_zeroes = m__io->read_u4le();
                m__io->seek(_pos);
                f_name_zeroes = true;
                return m_name_zeroes;
            }

            std::string microsoft_pe_t::annoyingstring_t::name_from_short() {
                if (f_name_from_short)
                    return m_name_from_short;
                n_name_from_short = true;
                if (name_zeroes() != 0) {
                    n_name_from_short = false;
                    std::streampos _pos = m__io->pos();
                    m__io->seek(0);
                    m_name_from_short = kaitai::kstream::bytes_to_str(m__io->read_bytes_term(0, false, true, false), std::string("ascii"));
                    m__io->seek(_pos);
                    f_name_from_short = true;
                }
                return m_name_from_short;
            }

            microsoft_pe_t::pe_header_t* microsoft_pe_t::pe() {
                if (f_pe)
                    return m_pe.get();
                std::streampos _pos = m__io->pos();
                m__io->seek(mz()->ofs_pe());
                m_pe = std::unique_ptr<pe_header_t>(new pe_header_t(m__io, this, m__root));
                m__io->seek(_pos);
                f_pe = true;
                return m_pe.get();
            }
        }
    }
}

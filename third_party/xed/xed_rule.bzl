
load("@bazel_skylib//lib:collections.bzl", "collections")
load("@rules_cc//cc:action_names.bzl", "C_COMPILE_ACTION_NAME")
load("@rules_cc//cc:toolchain_utils.bzl", "find_cpp_toolchain")


FOREIGN_CC_DISABLED_FEATURES = [
    "layering_check",
    "module_maps",
    "thin_lto",
]

def _configure_features(ctx, cc_toolchain):
    return cc_common.configure_features(
        ctx = ctx,
        cc_toolchain = cc_toolchain,
        requested_features = ctx.features,
        unsupported_features = ctx.disabled_features + FOREIGN_CC_DISABLED_FEATURES,
    )

def _file_name_no_ext(basename):
    (before, separator, after) = basename.rpartition(".")
    return before

def _files_map(files_list):
    by_names_map = {}
    for file_ in files_list:
        name_ = _file_name_no_ext(file_.basename)
        value = by_names_map.get(name_)
        if value:
            fail("Can not have libraries with the same name in the same category")
        by_names_map[name_] = file_
    return by_names_map

def _create_libraries_to_link(ctx, static_libraries, cc_toolchain, feature_configuration):
    libs = []

    static_map = _files_map(static_libraries or [])

    #static_map = _files_map(_filter(static_libraries or [], _is_position_independent, True))
    #pic_static_map = _files_map(_filter(static_libraries or [], _is_position_independent, False))

    names = collections.uniq(static_map.keys())

    for name_ in names:
        libs.append(cc_common.create_library_to_link(
            actions = ctx.actions,
            feature_configuration = feature_configuration,
            cc_toolchain = cc_toolchain,
            #static_library = static_map.get(name_),
            pic_static_library = static_map.get(name_),
            #dynamic_library = shared_map.get(name_),
            #interface_library = interface_map.get(name_),
            #alwayslink = ctx.attr.alwayslink,
        ))

    return depset(direct = libs)

def _compile_xed_impl(ctx):
    cc_toolchain = find_cpp_toolchain(ctx)
    
    outdir = ctx.actions.declare_directory('out')
    out_includedir = ctx.actions.declare_directory('out/include')
    
    mfile_args = []
    
    mfile_args += ["clean", "install"]
    mfile_args += ["--static"]
    mfile_args += ["--src-dir", ctx.files.source_root[0].path]
    mfile_args += ["--install-dir", outdir.path]

    mfile_args += ["--cc", cc_toolchain.compiler_executable]
    mfile_args += ["--cxx", cc_toolchain.compiler_executable]
    mfile_args += ["--as", cc_toolchain.compiler_executable] # will this work?
    mfile_args += ["--ar", cc_toolchain.ar_executable]
    mfile_args += ["--strip", cc_toolchain.strip_executable]
    mfile_args += ["--verbose=0"]
    
    libs = [
        "libxed.a",
        "libxed-ild.a",
    ]
    includes = ['xed-address-width-enum.h', 'xed-agen.h', 'xed-attribute-enum.h', 'xed-attributes.h', 'xed-build-defines.h', 'xed-category-enum.h', 'xed-chip-enum.h', 'xed-chip-features.h', 'xed-common-defs.h', 'xed-common-hdrs.h', 'xed-convert-table-init.h', 'xed-cpuid-bit-enum.h', 'xed-cpuid-rec.h', 'xed-decoded-inst-api.h', 'xed-decoded-inst.h', 'xed-decode.h', 'xed-disas.h', 'xed-encode-check.h', 'xed-encode-direct.h', 'xed-encode.h', 'xed-encoder-gen-defs.h', 'xed-encoder-hl.h', 'xed-encoder-iforms.h', 'xed-error-enum.h', 'xed-exception-enum.h', 'xed-extension-enum.h', 'xed-flag-action-enum.h', 'xed-flag-enum.h', 'xed-flags.h', 'xed-format-options.h', 'xed-gen-table-defs.h', 'xed-get-time.h', 'xed-iclass-enum.h', 'xed-iform-enum.h', 'xed-iformfl-enum.h', 'xed-iform-map.h', 'xed-ild.h', 'xed-immdis.h', 'xed-immed.h', 'xed-init.h', 'xed-init-pointer-names.h', 'xed-inst.h', 'xed-interface.h', 'xed-isa-set-enum.h', 'xed-isa-set.h', 'xed-machine-mode-enum.h', 'xed-nonterminal-enum.h', 'xed-operand-accessors.h', 'xed-operand-action-enum.h', 'xed-operand-action.h', 'xed-operand-convert-enum.h', 'xed-operand-ctype-enum.h', 'xed-operand-ctype-map.h', 'xed-operand-element-type-enum.h', 'xed-operand-element-xtype-enum.h', 'xed-operand-enum.h', 'xed-operand-storage.h', 'xed-operand-type-enum.h', 'xed-operand-values-interface.h', 'xed-operand-visibility-enum.h', 'xed-operand-width-enum.h', 'xed-patch.h', 'xed-portability.h', 'xed-print-info.h', 'xed-reg-class-enum.h', 'xed-reg-class.h', 'xed-reg-enum.h', 'xed-reg-role-enum.h', 'xed-rep-prefix.h', 'xed-state.h', 'xed-syntax-enum.h', 'xed-types.h', 'xed-util.h', 'xed-version.h']
    
    # is this required?
    # should I be declaring directories I do not care about?
    dirs = [
        outdir,
        ctx.actions.declare_directory('out/lib'),
        out_includedir,
        ctx.actions.declare_directory('out/include/xed')
    ]
    
    # same for files: is it required to declare unimportant files?
    libs = [ctx.actions.declare_file('out/lib/' + lib) for lib in libs]
    includes = [ctx.actions.declare_file('out/include/xed/' + include) for include in includes]
    outputs = dirs + libs + includes

    feature_configuration = _configure_features(
        ctx = ctx,
        cc_toolchain = cc_toolchain,
    )

    c_compile_variables = cc_common.create_compile_variables(
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
    )
    # get the env to make bazel_rules_cc_toolchain work
    env = cc_common.get_environment_variables(
        feature_configuration = feature_configuration,
        action_name = C_COMPILE_ACTION_NAME,
        variables = c_compile_variables
    )

    # TODO: get flags for C & C++ compilation and pass them to mbuild

    ctx.actions.run(
        inputs = ctx.files.all_sources + cc_toolchain.all_files.to_list(),
        outputs = outputs,
        arguments = mfile_args,
        executable = ctx.executable.mfile,
        env = env,
        
        mnemonic = "XedCompile",
        progress_message = "Compiling xed using its NIH build system",
    )
    
    compilation_info = cc_common.create_compilation_context(
        headers = depset([out_includedir]),
        system_includes = depset([out_includedir.path]),
        includes = depset([]),
        quote_includes = depset([]),
        defines = depset([]),
    )
    linking_info = cc_common.create_linking_context(
        linker_inputs = depset(direct = [
            cc_common.create_linker_input(
                owner = ctx.label,
                libraries = _create_libraries_to_link(ctx, libs, cc_toolchain, feature_configuration),
            ),
        ]),
    )
    
    cc_info = CcInfo(
        compilation_context = compilation_info,
        linking_context = linking_info,
    )
    return [
        DefaultInfo(files = depset(outputs)),
        cc_info
    ]


compile_xed = rule(
    implementation = _compile_xed_impl,
    toolchains = ["@bazel_tools//tools/cpp:toolchain_type"],
    fragments = ['cpp'],
    attrs = {
        "mfile": attr.label(
            mandatory = True,
            executable = True,
            cfg = "exec"
        ),    
        "all_sources": attr.label(
            mandatory = True
        ),
        "source_root": attr.label(
            mandatory = True,
            allow_single_file = True
        )
    }
) 

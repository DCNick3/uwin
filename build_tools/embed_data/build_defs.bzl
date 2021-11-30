# Copyright 2019 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""Embeds data files into a C++ module."""

load("@rules_cc//cc:toolchain_utils.bzl", "find_cpp_toolchain")
load("@rules_cc//cc:action_names.bzl", 
  "CPP_COMPILE_ACTION_NAME",
  "CPP_LINK_STATIC_LIBRARY_ACTION_NAME",
)
load("@bazel_skylib//lib:paths.bzl", "paths")

DISABLED_FEATURES = [
    "module_maps",
]

# like in https://github.com/bazelbuild/rules_cc/blob/main/examples/my_c_compile/my_c_compile.bzl
def _compile_object(ctx, source_file, output_file = None, additional_sources = depset()):
  cc_toolchain = find_cpp_toolchain(ctx)
  output_file = output_file or ctx.actions.declare_file(ctx.label.name + "_" + paths.basename(source_file.path) + ".o")
  feature_configuration = cc_common.configure_features(
    ctx = ctx,
    cc_toolchain = cc_toolchain,
    requested_features = ctx.features,
    unsupported_features = DISABLED_FEATURES + ctx.disabled_features,
  )
  cc_compiler_path = cc_common.get_tool_for_action(
    feature_configuration = feature_configuration,
    action_name = CPP_COMPILE_ACTION_NAME,
  )
  cc_compile_variables = cc_common.create_compile_variables(
    feature_configuration = feature_configuration,
    cc_toolchain = cc_toolchain,
    user_compile_flags = ctx.fragments.cpp.copts + ctx.fragments.cpp.conlyopts,
    source_file = source_file.path,
    output_file = output_file.path,
  )
  command_line = cc_common.get_memory_inefficient_command_line(
    feature_configuration = feature_configuration,
    action_name = CPP_COMPILE_ACTION_NAME,
    variables = cc_compile_variables,
  )
  env = cc_common.get_environment_variables(
    feature_configuration = feature_configuration,
    action_name = CPP_COMPILE_ACTION_NAME,
    variables = cc_compile_variables,
  )

  ctx.actions.run(
    executable = cc_compiler_path,
    arguments = command_line,
    env = env,
    inputs = depset(
        [source_file],
        transitive = [cc_toolchain.all_files, additional_sources],
    ),
    outputs = [output_file],
  )

  return output_file

def _link_static_library(ctx, objects, output_file = None):
  cc_toolchain = find_cpp_toolchain(ctx)
  output_file = output_file or ctx.actions.declare_file(ctx.label.name + ".a")
  feature_configuration = cc_common.configure_features(
    ctx = ctx,
    cc_toolchain = cc_toolchain,
    requested_features = ctx.features,
    unsupported_features = ctx.disabled_features,
  )

  archiver_path = cc_common.get_tool_for_action(
    feature_configuration = feature_configuration,
    action_name = CPP_LINK_STATIC_LIBRARY_ACTION_NAME,
  )
  archiver_variables = cc_common.create_link_variables(
    feature_configuration = feature_configuration,
    cc_toolchain = cc_toolchain,
    output_file = output_file.path,
    is_using_linker = False,
  )
  command_line = cc_common.get_memory_inefficient_command_line(
    feature_configuration = feature_configuration,
    action_name = CPP_LINK_STATIC_LIBRARY_ACTION_NAME,
    variables = archiver_variables,
  )
  args = ctx.actions.args()
  args.add_all(command_line)
  args.add_all(objects)

  env = cc_common.get_environment_variables(
    feature_configuration = feature_configuration,
    action_name = CPP_LINK_STATIC_LIBRARY_ACTION_NAME,
    variables = archiver_variables,
  )

  ctx.actions.run(
    executable = archiver_path,
    arguments = [args],
    env = env,
    inputs = depset(
      direct = objects,
      transitive = [
        cc_toolchain.all_files,
      ],
    ),
    outputs = [output_file],
  )

  return output_file

def _impl(ctx):
  cc_toolchain = find_cpp_toolchain(ctx)
  feature_configuration = cc_common.configure_features(
    ctx = ctx,
    cc_toolchain = cc_toolchain,
    requested_features = ctx.features,
    unsupported_features = DISABLED_FEATURES + ctx.disabled_features,
  )
  linker_path = cc_toolchain.ld_executable
  link_variables = cc_common.create_link_variables(
    cc_toolchain = cc_toolchain,
    feature_configuration = feature_configuration,
  )
  env = cc_common.get_environment_variables(
    feature_configuration = feature_configuration,
    action_name = CPP_LINK_STATIC_LIBRARY_ACTION_NAME,
    variables = link_variables,
  )

  example_obj = _compile_object(ctx, ctx.file._example_c_source)

  data_object = ctx.actions.declare_file(ctx.label.name + "_data.o")
  cpp_file = ctx.actions.declare_file(ctx.label.name + "_toc.cc")
  header_file = ctx.outputs.h_file_output or ctx.actions.declare_file(ctx.label.name + ".h")
  
  generate_embed = ctx.executable._generate_embed

  args = ctx.actions.args()
  args.add("-l", linker_path)
  args.add("--example-object", example_obj)
  args.add("--object", data_object)
  args.add("--cpp", cpp_file)
  args.add("--header", header_file)
  args.add("--namespace", ctx.attr.cpp_namespace)
  args.add("--embedname", ctx.label.name)
  args.add_all(ctx.files.srcs)

  ctx.actions.run(
    executable = generate_embed,
    arguments = [args],
    env = env,
    inputs = depset(
      ctx.files.srcs + [example_obj],
      transitive = [cc_toolchain.all_files],
    ),
    outputs = [data_object, cpp_file, header_file],
  )

  toc_object = _compile_object(ctx, cpp_file, None, depset([header_file]))

  archive = _link_static_library(ctx, [data_object, toc_object])

  linker_input = cc_common.create_linker_input(
    owner = ctx.label,
    libraries = depset(direct = [
      cc_common.create_library_to_link(
        actions = ctx.actions,
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        static_library = archive,
      ),
    ]),
  )
  compilation_context = cc_common.create_compilation_context(
    headers = depset([header_file]),
  )
  linking_context = cc_common.create_linking_context(linker_inputs = depset(direct = [linker_input]))

  return [
    CcInfo(
      compilation_context = compilation_context,
      linking_context = linking_context,
    )
  ]


cc_embed_data = rule(
  implementation = _impl,
  attrs = {
    "srcs": attr.label_list(
      mandatory = True,
      allow_empty = False, 
      allow_files = True,
      doc = "The files to embed",
    ),
    "cpp_namespace": attr.string(
      default = "",
      doc = "C++ namespace to put the retrieval function in",
    ),
    "h_file_output": attr.output(
      mandatory = False,
      doc = "Header file to generate. If not specified - use label name"
    ),
    "_example_c_source": attr.label(
      allow_single_file = True,
      default = "//build_tools/embed_data:dummy.cc",
      doc = "A file used to build example object to tell the linker of the object format expected from it",
    ),
    "_generate_embed": attr.label(
      executable = True,
      cfg = "exec",
      default = "//build_tools/embed_data:generate_embed",
    )
  },
  toolchains = ["@bazel_tools//tools/cpp:toolchain_type"],
  fragments = ["cpp"],
)

# def cc_embed_data(
#         name,
#         srcs,
#         cc_file_output,
#         h_file_output,
#         cpp_namespace = None,
#         strip_prefix = None,
#         flatten = False):
#     """Embeds 'srcs' into a C++ module.
#     Generates a header like:
#       namespace foo {
#       struct FileToc {
#         const char* name;             // the file's original name
#         const char* data;             // beginning of the file
#         size_t size;                  // length of the file
#       };
#       extern const struct FileToc* this_rule_name_create();
#       }
#     The 'this_rule_name()' function will return an array of FileToc
#     structs terminated by one that has nullptr 'name' and 'data' fields.
#     The 'data' field always has an extra null terminator at the end (which
#     is not included in the size).
#     Args:
#       name: The rule name, which will also be the identifier of the generated
#         code symbol.
#       srcs: List of files to embed.
#       cc_file_output: The CC implementation file to output.
#       h_file_output: The H header file to output.
#       cpp_namespace: Wraps everything in a C++ namespace.
#       strip_prefix: Strips this verbatim prefix from filenames (in the TOC).
#       flatten: Removes all directory components from filenames (in the TOC).
#     """
#     generator = "//build_tools/embed_data:generate_cc_embed_data"
#     generator_location = "$(location %s)" % generator
#     flags = "--output_header='$(location %s)' --output_impl='$(location %s)'" % (
#         h_file_output,
#         cc_file_output,
#     )
#     flags += " --identifier='%s'" % (name,)
#     if cpp_namespace != None:
#         flags += " --cpp_namespace='%s'" % (cpp_namespace,)
#     if strip_prefix != None:
#         flags += " --strip_prefix='%s'" % (strip_prefix,)
#     if flatten:
#         flags += " --flatten"

#     native.genrule(
#         name = name + "__generator",
#         srcs = srcs,
#         outs = [
#             cc_file_output,
#             h_file_output,
#         ],
#         tools = [generator],
#         cmd = "%s $(SRCS) %s" % (generator_location, flags),
#     )
#     native.cc_library(
#         name = name,
#         hdrs = [h_file_output],
#         srcs = [cc_file_output],
#     )
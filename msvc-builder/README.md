This tool uses wine to build test exe files with MSVC 6.0

You should run it from the root of the workspace (it will then access test_exes directory)

You should also get a hold of MSVC 6.0 to use for compilation purposes and set the path to it in an environment variable MSVC_PATH. The tool will check if the contents match with what it expects (including caches). It expects a distribution from microsoft, but slightly modified (all file names are lowercased, some files removed (TODO: which), maybe some more?)

You can also set WINEPREFIX env variable. If it is not set, the tool will create a temporary directory for a prefix, but this might be inconvenient for testing (wine prefix init is kidna long), so you can use WINEPREFIX to make it reuse the prefix.


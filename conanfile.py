from conans import ConanFile, CMake, tools, RunEnvironment
from pathlib import Path
import os
import shutil

class UwinConan(ConanFile):
    name = "uwin"
    version = "0.1"
    settings = "os", "compiler", "build_type", "arch"
    url = "https://gitlab.com/uwin-dev/uwin"
    description = """portable win32 emulation layer"""
    generators = "cmake"

    # those must work in the host context & are not required in order to run stuff, so they are build_requires
    build_requires = "uwin-remill/0.2@uwin/stable", \
                     "openwatcom-v2/2021-04-02@uwin/stable", \
                     "ghidra/9.2.3@uwin/stable"

    def _configure_cmake(self):
        cmake = CMake(self)
        cmake.configure(source_folder=".", )
        return cmake


    def build(self):
        cmake = self._configure_cmake()
        cmake.build()

    def package(self):
        cmake = self._configure_cmake()
        cmake.install()

# uwin

uwin is a (reasonably) portable runtime with implementation of some win32 implementations, aimed at running windows games from around '99.

While being somewhat similar to WINE project, uwin aims to also provide CPU emulation facilities (allowing it to run on non-x86 machines) and more portability (not only UNIX!)

## State

This project is very much work and progress and not much works yet.

Right now it contains `rusty-x86`: mostly functional (but not yet optimized) x86 static recompiler with an interpreter fallback, along with some groundwork for supporting win32 APIs (the coverage is not that good yet).

In terms of Win32 API support, it can run basic console programs and aims to implement support for at least DirectDraw and WinMM sound output. More APIs can come later

There are some notes at the [wiki](https://wikijs.dcnick3.me/)

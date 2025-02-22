# Tiny Rust libc

## Introduction

This is a _tiny_ libc implementation, mostly (but not entirely) written in the Rust programming language. It is useful for bare-metal embedded Rust applications that need a C library (maybe because of some third-party library written in C they want to use) but don't want to link against a full [newlib](https://sourceware.org/newlib), or who tried but had trouble with both newlib and [compiler_builtins](https://github.com/rust-lang-nursery/compiler-builtins) defining symbols like `memset`.

This crate basically came about so that the [nrfxlib](https://github.com/NordicPlayground/nrfxlib) binary interface library for the nRF9160 would work with Rust.

## Implemented so far

* abs
* strol
* atoi
* strcmp
* strncmp
* strcpy
* strncpy
* strlen
* strtol
* strtoul
* strstr
* strchr
* snprintf
* vsnprintf

## Non-standard helper functions

* itoa
* utoa

## To Do

* Anything else nrfxlib needs
* Anything anyone is prepared to submit

## Licence

As this is going to be a bunch of bits taken from all over the place (some newlib, some relibc, etc), each function has its own file and each file has its own licence. Any new licences should be appended to the [LICENCE.md](./LICENCE.md) file.


#!/usr/bin/env sh

bindgen 'src/wrapper.h' --ctypes-prefix '::libc' --no-unstable-rust > 'src/bindings.rs' -- $(pkg-config 'lua >= 5.3' --cflags-only-I)

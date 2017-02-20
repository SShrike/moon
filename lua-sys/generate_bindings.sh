#!/usr/bin/env sh
bindgen 'src/wrapper.h' --ctypes-prefix '::libc' --no-unstable-rust > 'src/bindings.rs'

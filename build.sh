#!/bin/sh

# Use
#   Builder.ctypes_prefix("cty") / --ctypes-prefix=cty
#   Builder.use_core() / --use-core
# to make the generated code #![no_std] compatible.
#bindgen cry/include/cry/cry.h --ctypes-prefix=cty --use-core -o src/bindings.rs
bindgen cry/include/cry/cry.h --use-core -o src/bindings.rs


# Build smile lib
cargo build

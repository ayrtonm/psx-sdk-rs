RUSTC = rustc

# Cross compilation toolchain used for linking
CROSS = /opt/psx-tools/bin/mipsel-unknown-elf-
LD = $(CROSS)ld

# Absolute path to this directory. It makes the rest of the Makefiles
# simpler but it sucks a bit. We should be able to get this path
# dynamically but I suck at Makefiles
SDK_ROOT = $(HOME)/src/psx-sdk-rs

# Path to libcore's lib.rs
LIBCORE_SRC=$(HOME)/src/rust/src/libcore/lib.rs

# Region for the resulting executable: NA, E or J
REGION = E

# Absolude path to target.json in this directory. We'll be able to
# replace it with a relative path once
# https://github.com/rust-lang/rust/issues/24666 is fixed
TARGET_FILE = $(SDK_ROOT)/target.json

# Common rust flags (both for runtime and apps)
RUSTFLAGS = -O --target=$(TARGET_FILE) -C soft-float

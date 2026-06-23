CRATE_NAME := godot_imgui_rust
TARGET_DIR := ../addons/godot-imgui-rust/target
WASM_PROFILE_DIR := $(TARGET_DIR)/wasm32-unknown-emscripten/debug

# Pinned toolchain — single source of truth. Bump deliberately, not via `rustup update`.
# Latest stable for: EH default (panic=abort path), the --export=__heap_base requirement, 
# and -Zbuild-std on emscripten.
NIGHTLY := nightly-2026-06-19

# PIC for the C/C++ deps. imgui-sys compiles Dear ImGui + cimgui (C++) via the cc crate.
# The threaded wasm build must compile that C++ with -pthread so emscripten's atomics /
# shared-memory features match the Rust objects; mismatched features fail to link.
# These are applied per-recipe (see build-threads / build-nothreads), not globally,
# because the threaded and nothreads passes need different values in one `make` run.
WASM_CC_NOTHREADS := -fPIC
WASM_CC_THREADS   := -fPIC -pthread

# Shared flags. panic=abort (in Cargo.toml) handles EH; no -wasm-use-legacy-eh needed.
RUSTFLAGS_COMMON := -C relocation-model=pic \
                    -C link-args=-sSIDE_MODULE=2 \
                    -C llvm-args=-enable-emscripten-cxx-exceptions=0 \
                    -Z default-visibility=hidden \
                    -Z link-native-libraries=no

# Threaded: + atomics, pthread, and --export=__heap_base (required on this nightly).
RUSTFLAGS_THREADS := $(RUSTFLAGS_COMMON) \
                     -C link-args=-pthread \
                     -C target-feature=+atomics \
                     -C link-arg=--export=__heap_base

RUSTFLAGS_NOTHREADS := $(RUSTFLAGS_COMMON)

CARGO := cargo +$(NIGHTLY)
CARGO_WASM := cargo +$(NIGHTLY)
WASM_ARGS := -Zbuild-std=std,panic_abort --target wasm32-unknown-emscripten

.PHONY: all ensure-toolchain native native-debug native-release \
        build-threads rename-wasm build-nothreads wasm clean

all: native wasm

ensure-toolchain:
	rustup toolchain install $(NIGHTLY) --profile minimal --component rust-src
	rustup target add wasm32-unknown-emscripten --toolchain $(NIGHTLY)

native: native-debug native-release
native-debug: | ensure-toolchain
	$(CARGO) build
native-release: | ensure-toolchain
	$(CARGO) build --release

wasm: build-nothreads

build-threads: | native
	CFLAGS_wasm32_unknown_emscripten="$(WASM_CC_THREADS)" \
	CXXFLAGS_wasm32_unknown_emscripten="$(WASM_CC_THREADS)" \
	RUSTFLAGS="$(RUSTFLAGS_THREADS)" $(CARGO_WASM) build $(WASM_ARGS)

rename-wasm: build-threads
	mv $(WASM_PROFILE_DIR)/$(CRATE_NAME).wasm \
	   $(WASM_PROFILE_DIR)/$(CRATE_NAME).threads.wasm

build-nothreads: rename-wasm
	CFLAGS_wasm32_unknown_emscripten="$(WASM_CC_NOTHREADS)" \
	CXXFLAGS_wasm32_unknown_emscripten="$(WASM_CC_NOTHREADS)" \
	RUSTFLAGS="$(RUSTFLAGS_NOTHREADS)" $(CARGO_WASM) build --features nothreads $(WASM_ARGS)

clean:
	$(CARGO_WASM) clean
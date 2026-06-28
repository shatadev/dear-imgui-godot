// Compiles the small C shim that formats Dear ImGui's printf-style error-recovery messages
// (see src/imgui_recover.c). Doing the varargs formatting in C keeps it portable across all
// targets, including wasm/emscripten, without Rust's unstable c_variadic feature.
fn main() {
    println!("cargo:rerun-if-changed=src/imgui_recover.c");
    cc::Build::new()
        .file("src/imgui_recover.c")
        .compile("imgui_recover");
}

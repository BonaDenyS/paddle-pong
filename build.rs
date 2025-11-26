fn main() {
    cc::Build::new()
        .file("c_engine/opengl_wrapper.c")
        .compile("opengl_wrapper_lib");
    println!("cargo:rerun-if-changed=c_engine/opengl_wrapper.c");
}

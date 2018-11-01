extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cimgui/cimgui.cpp")
        .file("cimgui/imgui/imgui.cpp")
        .file("cimgui/imgui/imgui_demo.cpp")
        .file("cimgui/imgui/imgui_draw.cpp")
        .file("cimgui/imgui/imgui_widgets.cpp")
        .compile("libcimgui.a");

    let bindings = bindgen::Builder::default()
        .header("cimgui_wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/cimgui.rs")
        .expect("Couldn't write bindings!");
}
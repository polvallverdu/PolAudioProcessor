use std::path::Path;

fn main() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    // This is the directory where I built librubberband
    // using their COMPLILE.md
    // meson build && ninja -C build
    // it produces a library in this directory
    // and here I am generating the path relative to the current crate root
    let rubberband_build = crate_root.join("libs");

    // Tell cargo where to search for libraries
    // If you have librubberband installed globally, you might not need this line
    println!("cargo:rustc-link-search={}", rubberband_build.to_str().unwrap());
    
    // Tell the cargo that we will need to link to rubberband library 
    // This might not be necessary if you plan to keep rubberband.rs with link directive
    println!("cargo:rustc-link-lib=rubberband");
}

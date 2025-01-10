fn main() {
    // Tell Cargo to link with SDL2 dynamically
    println!("cargo:rustc-link-lib=dylib=SDL2");

    // Tell Cargo where to find the vcpkg installation
    println!("cargo:rustc-link-search=native=C:\\vcpkg\\vcpkg\\installed\\x64-windows\\lib");

    // If you have any additional configuration or include paths, add them here
    println!("cargo:include=C:\\vcpkg\\vcpkg\\installed\\x64-windows\\include");
}
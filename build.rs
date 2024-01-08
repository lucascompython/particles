fn main() {
    // Linux
    println!("cargo:rustc-link-search=native=pkg/raylib/usr/lib");

    // Windows
    //println!("cargo:rustc-link-search=native=raylib/lib");
    println!("cargo:rustc-link-lib=static=raylib");
}

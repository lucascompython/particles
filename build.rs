fn main() {
    println!("cargo:rustc-link-search=native=pkg/raylib/usr/lib");
    println!("cargo:rustc-link-lib=static=raylib");
}

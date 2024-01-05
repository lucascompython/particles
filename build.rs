fn main() {
    println!("cargo:rustc-link-search=native=/usrlib/");
    println!("cargo:rustc-link-lib=raylib");
}

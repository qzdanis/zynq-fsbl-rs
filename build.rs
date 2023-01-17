fn main() {
    println!("cargo:rustc-link-arg=-Tzynq.ld");

    build_libps7();
}

#[cfg(feature = "arty-z7")]
fn build_libps7() {
    cc::Build::new()
        .includes(["include", "include/arty_z7"])
        .files(["src/arty_z7/ps7_init.c"])
        .warnings(false)
        .compile("ps7");
}

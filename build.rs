fn main() {
    println!("cargo::rerun-if-changed=bitmap");
    cc::Build::new().file("bitmap/bmp.c").compile("bitmap");
}

use std::process::Command;
use cc;

fn main() {
    cc::Build::new()
        .include("cry/include")
        .file("cry/src/misc.c")
        .file("cry/src/crypt/base64.c")
        .compile("cry");

    let mut child = Command::new("make")
        .arg("all")
        .spawn()
        .unwrap();
    let _res = child.wait().unwrap();

    eprintln!("Hello");
}

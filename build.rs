use std::fs;
use std::process::Command;

fn main() {
    let status = Command::new("wasm-pack")
        .args(&[
            "build",
            "lumi-web",
            "--target",
            "web",
            "--no-typescript",
            "--out-dir",
            "web",
        ])
        .status()
        .unwrap();
    assert!(status.success());
    fs::copy("lumi-web/static/index.html", "lumi-web/web/index.html").unwrap();
    fs::copy("lumi-web/static/style.css", "lumi-web/web/style.css").unwrap();
    println!("cargo:rerun-if-changed=lumi-web/src");
    println!("cargo:rerun-if-changed=lumi-web/static/style.css");
    println!("cargo:rerun-if-changed=lumi-web/static/index.html");
}

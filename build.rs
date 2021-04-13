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
            "./static",
        ])
        .status()
        .unwrap();
    assert!(status.success());
    println!("cargo:rerun-if-changed=lumi-web/src");
    println!("cargo:rerun-if-changed=lumi-web/static/style.css");
    println!("cargo:rerun-if-changed=lumi-web/static/index.html");
}

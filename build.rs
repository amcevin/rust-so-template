use std::{env, path::Path};

fn main() {
    println!("cargo:warning=Hello, world!");

    let lib_name = env::var("CARGO_PKG_NAME").unwrap();
    let version = env!("CARGO_PKG_VERSION");
    // 只在 release 模式下处理
    let profile = env::var("PROFILE").unwrap();
    if profile != "release" {
        return;
    }

    // 判断平台
    #[cfg(target_os = "macos")]
    let ext = "dylib";
    #[cfg(not(target_os = "macos"))]
    let ext = "so";

    let target_dir = Path::new("target").join(&profile);
    let src = target_dir.join(format!("lib{}.{}", lib_name, ext));
    let dst = target_dir.join(format!("lib{}.{}.{}", lib_name, version, ext));

    println!("cargo:warning=src: {}", src.display());
    println!("cargo:warning=dst: {}", dst.display());

    // 重命名
    std::fs::rename(src, dst).unwrap();
}

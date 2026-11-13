use std::path::Path;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(clima_icon_asset)");
    println!("cargo:rustc-check-cfg=cfg(clima_emoji_font)");
    println!("cargo:rerun-if-changed=assets");

    if Path::new("assets/icon.png").is_file() {
        println!("cargo:rustc-cfg=clima_icon_asset");
    }
    if Path::new("assets/NotoEmoji-Regular.ttf").is_file() {
        println!("cargo:rustc-cfg=clima_emoji_font");
    }
}

fn main() {
    println!("cargo:rustc-env=TAURI_DIR=../");
    tauri_build::build()
}

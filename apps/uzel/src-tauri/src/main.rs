#![forbid(unsafe_code)]

fn main() {
    tauri::Builder::default()
        .setup(|_| {
            println!("UZEL_SHELL_READY");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Uzel shell failed");
}

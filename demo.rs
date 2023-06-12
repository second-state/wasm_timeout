// rustup target add wasm32-wasi
// rustc --target=wasm32-wasi -O demo.rs
fn main() {
    let id = std::env::var("id");
    println!("[wasm] hello world {id:?}");
    println!("[wasm] wait 3s...");
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("[wasm] start loop");
    loop {}
    println!("[wasm] bye {id:?}");
}

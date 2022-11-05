// implement util functions export to C
#[no_mangle]
pub extern "C" fn dummy() -> i32 {
    println!("Hello, world from Rust!");
    0
}

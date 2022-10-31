use wowsunpacker::ParamsUnpacker;


fn main() {
    // run this from /target/debug, DLLs are placed there, cargo run doesn't work
    let unpacker = ParamsUnpacker::new().unwrap();
    unpacker.unpack("../../output/content/GameParams.data", false).unwrap();
}

use wowsunpacker::ParamsUnpack;


fn main() {
    // run this from /target/debug, DLLs are placed there, cargo run doesn't work
    ParamsUnpack("../../output/content/GameParams.data", false).unwrap();
}

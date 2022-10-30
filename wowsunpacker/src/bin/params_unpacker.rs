use wowsunpacker::params_unpack::params_unpack;

fn main() {
    // run this from /target/debug, DLLs are placed there, cargo run doesn't work
    params_unpack("../../output/content/GameParams.data", false).unwrap();
}

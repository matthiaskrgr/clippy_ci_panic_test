// this simulates clippy panicking due to ICE
// if we do not catch this in our CI; raise an error

pub fn it_looks_like_you_are_trying_to_kill_clippy() {}
fn main() {}

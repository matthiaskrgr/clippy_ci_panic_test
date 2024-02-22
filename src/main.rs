// this simulates clippy panicking due to ICE
// if we do not catch this in our CI; raise an error

#![allow(internal_features)]
#![feature(rustc_attrs)]
#[rustc_error(delayed_bug_from_inside_query)]
fn main() {}

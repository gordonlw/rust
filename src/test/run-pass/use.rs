// xfail-test
use std;
use libc;
use zed(name = "std");
use bar(name = "std", vers = "0.1");


// FIXME: commented out since resolve doesn't know how to handle crates yet.
// import std::str;
// import x = std::str;
mod baz {
    // import std::str;
    // import x = std::str;

}

fn main() { }
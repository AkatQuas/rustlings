// modules3.rs
// You can use the 'use' keyword to bring module paths from modules from anywhere
// and especially from the Rust standard library into your scope.
// Bring SystemTime and UNIX_EPOCH
// from the std::time module. Bonus style points if you can do it with one line!
// Make me compile! Execute `rustlings hint modules3` for hints :)
use std::time::*;
// or
// use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}, s1 is {}, s2 is {}", s, s1, s2);
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

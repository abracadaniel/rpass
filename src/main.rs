use std::env;
use rand::thread_rng;

const DEFAULT_LEN: usize = 32;

fn main() {
    // Initialize strength level: >1 adds special characters
    let lvl: usize = match env::args().nth(1) {
        Some(s) => { match s.parse() { Ok(s) => s, Err(_) => 0} },
	    None => { 0 },
    };

    // Initialize length and get custom length from argument
    let len: usize = match env::args().nth(2) {
        Some(s) => { match s.parse() { Ok(s) => s, Err(_) => DEFAULT_LEN} },
	    None => { DEFAULT_LEN },
    };

    // Define charsets
    let mut charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        abcdefghijklmnopqrstuvwxyz\
        0123456789".to_vec();
    let special = b")(*&^%$#@!~[]{}";

    // Add special chars according to strength level
    if lvl > 0 {
        charset.extend_from_slice(special);
    }

    // Generate password
    let mut pass = String::new();
    while pass.len() < len {
        pass.push(charset.choose(&mut thread_rng()).cloned().unwrap().into());
    }

    println!("{}", pass);
}
use std::io::{self, BufRead};

pub fn open_file() -> impl BufRead {
    let stdin = io::stdin();
    stdin.lock()
}

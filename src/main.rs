use std::fs;
use std::hash::Hasher;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use failure::Error;
use twox_hash::XxHash;

fn main() {
    //let hasher = XxHash::default();
    let hash = hash_file("/Users/jonathan.fokkan/base64.go").expect("hash file");
    println!("hash: {}", hash);
}

pub fn hash_file<P: AsRef<Path>>(file_path: P) -> Result<String, Error> {
    let now = Instant::now();
    let mut hasher = XxHash::default();
    let file = fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);
    loop {
        let length = {
            let buf = reader.fill_buf()?;
            // do stuff with buffer here
            hasher.write(buf);
            buf.len()
        };
        if length == 0 {
            break;
        }
        reader.consume(length);
    }
    println!("Time taken to hash: {}s", now.elapsed().as_secs());
    let hashed_value = hasher.finish();
    Ok(format!("{:x}", hashed_value))
}

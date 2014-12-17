
extern crate "rust-crypto" as crypto;

use crypto::md5::Md5 as Md5;
use crypto::digest::Digest;

pub fn cal_md5(v: & [u8] ) -> Option<String> {
    let mut sh = Md5::new();
    let len = v.len();
    let mut left = len;

    while left > 0u {
        let take = (left + 1u) / 2u;
        sh.input(v.slice(len - left, take + len - left));
        left = left - take;
    }

    let ret = sh.result_str();
    return Some(ret);
}

fn cal_str() {
    let z = "a"; 
    let mut sh = Md5::new();
    sh.input_str(z);
    let out = sh.result_str();
    sh.reset();
    println!("md5 of a is {}",out);
}

fn main() {
    use std::io::File;

    cal_str();
    let z = "a".as_bytes();
    cal_md5( z );
    let path = Path::new("./ff");
    let mut file = File::open(&path);
    let content = file.read_to_end().unwrap();
    cal_md5(content.as_slice());
}

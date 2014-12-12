
extern crate "rust-crypto" as crypto;

use crypto::md5::Md5 as Md5;
use crypto::digest::Digest;
use std::fmt::radix;

use std::io::fs::PathExtensions;
use std::str;
use std::io::File;
use std::io;


pub fn cal_md5(v: & [u8] ) -> Option<String> {
    let mut sh = Md5::new();
    let len = v.len();
    let mut left = len;

    while left > 0u {
        let take = (left + 1u) / 2u;
        sh.input(v.slice(len - left, take + len - left));
        left = left - take;
    }

    let mut sh_ret: Vec<u8> = Vec::from_elem(16, 0u8);
    let mut t2 = sh_ret.as_mut_slice();
    let mut v :Vec<String> = Vec::new();

    let mut zz = sh.result_str();
    println!("zz   result is {}", zz);
    sh.result(t2);
    println!(" out is {}", t2);
    sh.reset();

    let mut v = Vec::new();
    for &i in t2.iter() {
        let mut s ; 
        let mut hex :  std::fmt::RadixFmt<u8, std::fmt::Radix>; //std::fmt::Radix;
        if i > 15 {
            hex = std::fmt::radix(i, 16);
            s = format!("{:2}" , hex);
        } else {
            hex = std::fmt::radix(i, 16);
            s = format!("0{}" , hex);
        }
        v.push(s);
    }
    let mut v = v.concat();
    println!("{}",v);
    return Some(v);
}


fn cal_str() {
    let mut z = "a"; 
    let mut sh = Md5::new();
    sh.input_str(z);
    let mut t: Vec<u8> = Vec::new();
    let tt = t.as_mut_slice();
    let mut out = sh.result_str();
    sh.reset();
    println!("md5 of a is {}",out);
}

fn main() {
    
    cal_str();
    let sh = Md5::new();
    let z = "a".as_bytes();
    cal_md5( z );
    let path = Path::new("./ff");
    let mut file = File::open(&path);
    let content = file.read_to_end().unwrap();
    cal_md5(content.as_slice());

}

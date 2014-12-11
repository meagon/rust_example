#![feature(slicing_syntax)] 


fn main(){


    let mut z = vec![1u8,2,3,4];

    let mut zz = z[0..z.len()];
    println!("{}",zz);
}

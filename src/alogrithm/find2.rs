extern crate collections;
use std::collections::VecMap;
use std::collections::HashMap;

fn find_boyer(t: Vec<u8>,p :Vec<u8>, start: uint) -> Option<uint>{
    let mut n =t.len();
    let mut m =p.len();
    let mut last = HashMap::new();
    for i in range(0, m){
        last.insert(p[i] as uint,i);
    }
    let mut i = start + m -1;
    let mut k = m -1;
    while ( i < n){
        println!("at loop...")
            if (t[i] == p[k] ){
                if t[i-m..i-1] == p[0..m-1]{
                    return Some(i-m);
                }
            }
            /*
        if (t[i] == p[k] ){
            println!("at if loop...")
            if k == 0u {
                return Some(i)
            }
            else{
                i = i-1;
                k = k-1;
            }
        }
        */
        /*    
        if (t[..i] == p[]
        }
        */
        else{
            println!("at else loop...  ");
            let j = last.get(&(t[i] as uint));
            println!("found j = {},   k ={}",j,k);
            let mut j_v :int;
            match j {
                Some(j) => { j_v = *j  as int;},
                None => { j_v = -1i ;},
            }
            // j = 1 , i = 2   , i =3
            i = i+ (m - std::cmp::min(k , (j_v+1) as uint) ) ;
            k = m-1;
            println!("i is {}" ,i);
        }
    }
    return None;
}

fn main(){

    let t = vec![1,2,3,4,5,6,2,3,4];
    let p = vec![2,3,4];
    let z = find_boyer(t,p,5u);
    println!("{}", z);
}




#![feature(slicing_syntax)] 
#![feature(phase)]
#[phase(plugin, link)] extern crate log;





pub mod search {


use std::collections::HashMap;

fn skip(t: &Vec<u8>, hash: & mut HashMap<u8,uint> ) {
    //v.reverse();
    //let mut hash = HashMap::new();

    let mut pos :u8 = 0;
    for i in t.iter(){
        hash.insert(*i as u8, t.len() - pos as uint -1);
        pos +=1;
    }
}


pub fn byyel(s :&Vec<u8>, t: &Vec<u8>, start:uint) -> Vec<uint>{

    let n = s.len();
    let m = t.len();

    let mut r = n -m ;
    let mut i = 0u + start;
    let mut k :uint;

    let mut hash :HashMap<u8,uint>= HashMap::new();
    skip(t, & mut hash);
    let mut element_position : Vec<uint> = Vec::new();
    while ( i <= n-m ){
        debug!("i = {}",i);
        debug!("at while loop..")
        if (s[i+m-1] == t[m-1]){
            if (s[i..i+m-1] == t[0..m-1]) {
                //return Some(i);
                println!("{}", s[i..i+m-1]);
                println!("{}", t[0..m-1]);
                element_position.push(i);
                i +=1;
            }
            else if (t.get(s[i+m] as uint ) == None) {
                debug!("at nop 1");
                i = i+m -1;
            }
            else {
                let mut element = t.get(s[i+m] as uint).unwrap();
                let  skip= *hash.get( element ).unwrap() as uint;
                i = i+ skip;
            }
            }
       else{
            debug!("...at else..and  s[i+m] = {}", s[i+m]);
            if (t.get(s[i+m] as uint) == None ){
                debug!("at nop 2");
                i = i +m ;
            }
            else {
                i = i+1;
            }
       }
        debug!("source {}",s[i..n]);
        debug!("dest   {}",t)
        debug!("i ={}   and n-m = {}", i, n-m);
   }
   //return None;
   return element_position;
}

}

fn main(){

    let mut s = vec![1u8,2,3,4,5,6,7,8,8,8,4,0,4,0,8,8,0];
    let mut t = vec![8u8,8,8,4,0,4];   
    let z =  search::byyel(&s,&t, 0);
    println!("{}",z);
}

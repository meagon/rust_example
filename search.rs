

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
    for i in hash.iter(){
        debug!("{}",i);
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
    while ( i < n-m ){
        debug!("i = {}",i);
        debug!("at while loop..")
        if (s[i+m-1] == t[m-1]){
            if (s[i..i+m-1] == t[0..m-1]) {
                //return Some(i);
                debug!("s is {}", s[i..i+m-1]);
                debug!("t us {}", t[0..m-1]);
                element_position.push(i);
                i +=m;
            }
            else if ( hash.get( &s[i+m] ) == None) {
                debug!("at nop 1");
                debug!("move {}", m-1)
                i = i+m -1;
            }
            else {
                debug!("at else one ");
                let mut element =  s[i+m];  //t.get(s[i+m] as uint).unwrap();
                let  skip= *hash.get( &element ).unwrap() as uint;
                i = i+ skip + 1;

                debug!("move {}",skip);
            }
       }
       else{
            debug!("...at else..and  s[i+m] = {}", s[i+m]);
            debug!("t get {}  {}",  s[i+m] , t.get(s[i+m] as uint) );
            if ( hash.get(& s[i+m]) == None ){
                debug!("at nop 2");
                i = i +m +1;
                debug!("move {}", m+1)
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

    let mut s = vec![1u8,2,3,4,5,6,7,13,10,13,10,13,4,0,8,8,0,9,5,6,7,8,9,4,8,8,4,4,4,8,8,8,4,0,4];
    let mut t = vec![10u8,13,10,13];   
    let z =  search::byyel(&s,&t, 0);
    debug!("{}",z);
}


#![feature(unboxed_closures)]

extern crate capnp;
extern crate collections;

// use std::collections::String;

use capnp::serialize_packed::{PackedOutputStream, PackedInputStream};

pub mod data_capnp{
    include!(concat!(env!("OUT_DIR"),"/data_capnp.rs"))
}


pub mod data {
    extern crate capnp;
    use std::io::{stdin, stdout, IoResult};
    use data_capnp::{bulk,data_object};
    use capnp::serialize_packed;
    use capnp::serialize;
    use capnp::{MessageBuilder,MessageReader,ReaderOptions,MallocMessageBuilder};
    use capnp::serialize_packed::{PackedOutputStream, PackedInputStream};
    
    use std::vec::Vec;
    use std::io::{BufferedWriter, File};
    use std::io::MemReader;
    use std::io::BufferedStream;
    use std;
    // pub fn write()  ->  IoResult<()> { //  Vec<u8>  {  //-> &'static [u8] {
    pub fn write()  ->   Vec<u8>  {  //-> &'static [u8] {
        let mut message = MallocMessageBuilder::new_default();
        {
            let mut data_obj = message.init_root::<data_object::Builder>();
            data_obj.set_key("hello");
            data_obj.set_data("somebinary_data".to_string().as_bytes());
            
           // println!( "{}" , std::mem::size_of_val(& data_obj) );
        }

        // return message;
        //let mut buf = String::new(); 
        let mut buf :Vec<u8> = Vec::new();
        // serialize::write_message(&mut buf, &message);
        // println!("{}", buf.as_slice());
        // println!("{}", String::from_utf8_lossy(buf.as_slice()) );
        // serialize_packed::write_packed_message_unbuffered(&mut stdout(), &message)
        //return buf;
        // struct S;
        // let mut stream = BufferedStream::new(buf);
        serialize_packed::write_packed_message_unbuffered(&mut buf, &message);
        return  buf;
        //serialize::write_message(&mut buf, &message);
    }

    pub fn read_raw(input : capnp::message::MallocMessageBuilder){
    }

    pub fn read(input : Vec<u8> ) -> IoResult<()> {

        // let mut in_data = input.container_as_bytes();
        // let mut in_data = input.into_boxed_slice();
        // println!("{}", input);
        // let mut in_data = input.map(|i| i as uint);
        let mut in_data = MemReader::new(input);

        // return vec!(10u);
        // let mut in_data = input.as_bytes();
        // let mut in_data :Box<[u8]> =  box in_data;
        // println!("{}", in_data.into_boxed_slice());
        // let mut c = Vec::new();
        let mut message_reader = try!(serialize_packed::new_reader_unbuffered(& mut in_data, ReaderOptions::new()));
        
        //match message_reader {
         //   Ok(message) => {println!("{}", "ok");
                //let data_obj = message.get_root_unchecked(1); // ::<data_object::Reader>();
                let data_obj = message_reader.get_root::<data_object::Reader>();
                let mut k = data_obj.get_key();
                println!("hello");
                println!("key is {}", k);
                let mut z = data_obj.get_data();
                //println!("{}",String::from_utf8_lossy(z));
            
            //}
            // Err(e) => {panic!("error");}
        // }
        Ok(())
    }
    /*
       xx//println!("{}", message_reader);
            Ok(message_reader) => {
            // let message_reader = serialize::new_reader(in_data, ReaderOptions::new()).unwrap();
            let data_obj  = message_reader.get_root::<data_object::Reader>();
            let k = data_obj.get_key();
            let file = data_obj.get_data();
            println!("{}", k);
            println!("{}", file);
            }
            Err(err) => { panic!(err);}
        }
    */ 
   
}


pub struct Buffer<T> { buf: Vec<T> }

pub type ObjectResult<T> = Result<T, ObjectStorageError>;  

#[deriving(Show)]
pub struct ObjectStorageError {
    pub code: int,
    message: String,
}

fn main() {

    let (tx, rx) = channel();
    let args = std::os::args();

    spawn ( proc() {
        loop { 
            let mut z = data::write();
            tx.send ( z );
        }
    });
    spawn ( proc() { 
        loop {
            let mut z = rx.recv();
            data::read(z);
        }
    });
}
    /*
    match args[1].as_slice() {
        "write" => data::write(),
        "read"  => data::read().unwrap(),
        _ => { println!("eror") }
    }
    */

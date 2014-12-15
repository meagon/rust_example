
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


    /// 结构化数据
    /// generate the serialized data bytes, try nano send it;
    /// and store it in backend storage
    /// if generate error, return None  ; 
    
    pub fn write( key: &str, data: Vec<u8>)  ->   Vec<u8>  {  //-> &'static [u8] {
        let mut message = MallocMessageBuilder::new_default();
        {
            let mut data_obj = message.init_root::<data_object::Builder>();
            data_obj.set_key( key );
            data_obj.set_data( data.as_slice() );
        }

        let mut buf :Vec<u8> = Vec::new();
        serialize_packed::write_packed_message_unbuffered(&mut buf, &message);
        return  buf;
    }

    pub fn read_raw(input : capnp::message::MallocMessageBuilder){
    }


    /* 解析数据并存储 will be useful for data storage nodes */
    pub fn read(input : Vec<u8> ) -> IoResult<()> {
        let mut in_data = MemReader::new(input);
        let mut message_reader = try!(serialize_packed::new_reader_unbuffered(& mut in_data, ReaderOptions::new()));
        //match message_reader {
         //   Ok(message) => {println!("{}", "ok");
                //let data_obj = message.get_root_unchecked(1); // ::<data_object::Reader>();
                let data_obj = message_reader.get_root::<data_object::Reader>();
                let mut k = data_obj.get_key();
                println!("hello");
                println!("key is {}", k);
                let mut z = data_obj.get_data();
            
        Ok(())
    }
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
            let mut data = "somebinary_data".to_string().into_bytes();
            let mut z = data::write( "hello", data);
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

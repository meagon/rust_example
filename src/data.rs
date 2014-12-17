
#![feature(unboxed_closures)]

extern crate capnp;
extern crate collections;

// use std::collections::String;
// 定义传输中的数据
//
use capnp::serialize_packed::{PackedOutputStream, PackedInputStream};

pub mod data_capnp{
    include!(concat!(env!("OUT_DIR"),"/data_capnp.rs"))
}

public struct Store{
    key: String,
    value: Vec<u8>,
}

pub struct Buffer<T> { buf: Vec<T> }
pub type ObjectResult<T> = Result<T, ObjectStorageError>;  
#[deriving(Show)]
pub struct ObjectStorageError {
        pub code: int,
        message: String,
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

    /// 结构化数据
    /// generate the serialized data bytes, try nano send it;
    /// and store it in backend storage
    /// if generate error, return None  ; 
    
    pub fn pack( key: &str, data: Vec<u8>)  ->   Vec<u8>  {  //-> &'static [u8] {
        let mut message = MallocMessageBuilder::new_default();
        {
            let mut data_obj = message.init_root::<data_object::Builder>();
            let mut key = key.into_string();
            data_obj.set_key( key );
            data_obj.set_data( data.as_slice() );
        }

        let mut buf :Vec<u8> = Vec::new();
        serialize_packed::write_packed_message_unbuffered(&mut buf, &message);
        return  buf;
    }

    /* 解析数据并存储 will be useful for data storage nodes */
    pub fn unpack(input : Vec<u8> ) -> Option<Store> {
        let mut in_data = MemReader::new(input);
        let mut message_reader = serialize_packed::new_reader_unbuffered(& mut in_data, ReaderOptions::new());
        match message_reader {
            Ok(message) = > {
                let data_obj = message.get_root::<data_object::Reader>();
                let k = data_obj.get_key();
                let v = data_obj.get_data();
                return Some(Store{key: k, value: v });
            },
            Err(e) => {return None},
        };
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

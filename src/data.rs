
#![feature(unboxed_closures)]

#![feature(phase)]
#[phase(plugin, link)] extern crate log;


extern crate capnp;

// use std::collections::String;
// 定义传输中的数据
//
//use capnp::serialize_packed::{PackedOutputStream, PackedInputStream};

pub mod data_capnp{
    include!(concat!(env!("OUT_DIR"),"/data_capnp.rs"))
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
    extern crate log;
    use data_capnp::{bulk,data_object};
    use capnp::serialize_packed;
    use capnp::{MessageBuilder,MessageReader,ReaderOptions,MallocMessageBuilder};
    use capnp::serialize_packed::{PackedOutputStream, };
    
    use std::vec::Vec;
    use std::io::MemReader;

    /// 结构化数据
    /// generate the serialized data bytes, try nano send it;
    /// and store it in backend storage
    /// if generate error, return None  ; 
    ///
    pub struct Store{
        key: String,
        value: Vec<u8>,
    }

    pub fn get_key( s: &Store) -> String {
        return s.key.clone();
    }

    pub fn show( s: &Store)  {
        println!("{}", s.key);
        println!("{}", String::from_utf8(s.value.clone()));
    }

    pub fn get_value(s: &Store) -> Vec<u8>{
       return s.value.clone(); 
    }
   
    /*will be useful for data gateway nodes */
    pub fn pack( key: String, data: Vec<u8>)  ->   Vec<u8>    {
        let mut message = MallocMessageBuilder::new_default();
        {
            let data_obj = message.init_root::<data_object::Builder>();
            let key = key.as_slice();
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
        let message_reader = serialize_packed::new_reader_unbuffered(& mut in_data, ReaderOptions::new());
        match message_reader {
            Ok(message) => {
                let data_obj = message.get_root::<data_object::Reader>();
                let k = data_obj.get_key().to_string();
                let v = data_obj.get_data().to_vec();
                return Some(Store{key: k, value: v });
            },
            Err(e) => { debug!( "{}", e);return None},
        };
    }
}


//#[test]
fn main() {

    let (tx, rx) = channel();
    spawn (move || {
        loop { 
            let data = "somebinary_data".to_string().into_bytes();
            let  z = data::pack( "hello".to_string(), data);
            tx.send ( z );
        }
    });
    spawn (move || { 
        loop {
            let z = rx.recv();
            let ret = data::unpack(z).unwrap();
            let k = data::get_key(&ret);
            let v = data::get_value(&ret);
            println!("{}", k);
            println!(" value  = {}", v);

        }
    });
}

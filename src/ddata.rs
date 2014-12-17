
#![feature(unboxed_closures)]

extern crate capnp;
extern crate collections;

// use std::collections::String;

use capnp::serialize_packed::{PackedOutputStream, PackedInputStream};

pub mod data_capnp {
    include! (concat ! ( env ! ( "OUT_DIR" ) , "/data_capnp.rs" ))
}

pub mod data {
    extern crate capnp;
    use std::io::{stdin, stdout, IoResult};
    use data_capnp::{bulk, data_object};
    use capnp::serialize_packed;
    use capnp::serialize;
    use capnp::{MessageBuilder, MessageReader, ReaderOptions,
                MallocMessageBuilder};
    use capnp::serialize_packed::{PackedOutputStream, PackedInputStream};
    use std::vec::Vec;
    use std::io::{BufferedWriter, File};
    use std::io::MemReader;
    use std::io::BufferedStream;

    /// generate the serialized data bytes, try nano send it;
    /// if generate error, return None; 
    pub fn write(key: &str, data: Vec<u8>) -> Vec<u8> { //-> &'static [u8] {
        let mut message = MallocMessageBuilder::new_default();
        {
            let mut data_obj = message.init_root::<data_object::Builder>();
            data_obj.set_key(key);
            data_obj.set_data(data.as_slice());
        }

        let mut buf: Vec<u8> = Vec::new();
        serialize_packed::write_packed_message_unbuffered(&mut buf, &message);
        return buf;
    }

    pub fn read_raw(input: capnp::message::MallocMessageBuilder) { }

    pub fn read(input: Vec<u8>) -> Option<Vec<u8>>  {

        let mut in_data = MemReader::new(input);
        let mut message_reader = try!(serialize_packed::new_reader_unbuffered (& mut in_data , ReaderOptions :: new () ));
        let data_obj = message_reader.get_root::<data_object::Reader>();
        let mut k = data_obj.get_key();
        println!("key is {}" , k);
        let mut z = data_obj.get_data();
        println!("value is {}", z.to_vec());
        Some((k,z.to_vec()))
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


pub struct Buffer<T> {
    buf: Vec<T>,
}
pub type ObjectResult<T> = Result<T, ObjectStorageError>;
#[deriving(Show)]
pub struct ObjectStorageError {
    pub code: int,
    message: String,
}



fn main() {

    let (tx, rx) = channel();
    let args = std::os::args();
    spawn(proc() {
          loop  {
              let mut data = "somebinary_data".to_string().into_bytes();
              let mut z = data::write("hello", data);
              tx.send(z);
          } });
    spawn(proc() { loop  { let mut z = rx.recv(); data::read(z); } });
}

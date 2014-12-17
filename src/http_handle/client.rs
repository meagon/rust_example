
#![feature(slicing_syntax)]

extern crate nanomsg;
extern crate data;

use self::nanomsg::{Socket, Protocol};
use std;

/*
pub fn init_client(uri: &str , rx: &std::comm::Receiver , ) {
    let mut socket = Socket::new(Protocol::Pair).unwrap();
    socket.connect(uri).unwrap();
    let mut endpoint = socket.connect(uri).unwrap();
    loop  {
         let mut data_need_send = rx.recv();
         let mut key : String = data_need_send.md5_sum();
         let  data: Vec<u8> = data_need_send;
         let  pack = data::data::pack( key, data);
         let  send_data = pack.as_slice();
         socket.write(b"hello");
    }
}
*/

pub struct gateway_client {
    remote_socket: nanomsg::Socket;
}

impl gateway_client {
    
        pub fn new(uri: : &str) -> Gate_client {
            let socket = Socket::new(Protocol::Pair).unwrap();
            let endpoint = socket.connect(uri).unwrap();
            gateway_client{ remote_socket :socket }
        }

        pub fn send( &self, data: Vec<u8>) -> Vec<u8> {
            self.remote_socket.write( data.as_slice() );
            let replay = self.remote_socket.read_to_end().unwrap();
            return replay;  
    }
}

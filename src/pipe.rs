#![feature(slicing_syntax)]

extern crate nanomsg;
extern crate data;

use nanomsg::{Socket, Protocol};

use utils;

pub fn worker(uri: &str) {
	let mut socket = Socket::new(Protocol::Pull).unwrap();
	let mut endpoint = socket.bind(uri);

    match endpoint {
       Ok(ok)   => {},
       Err(err) => {panic!("{}", err)},
    };  
    // [112, 117, 116]  put
    // [103, 101, 116]  get
    // [100, 101, 108]  del
    // [99, 111, 109]   com
    //
	// let mut endpoint = socket.bind(uri).unwrap();
	loop {
		let msg = socket.read_to_end().unwrap();
        //  serialize_unpack ..
        
        if msg.len() > 3u {
            match msg[0..3] {
             [112u8,117,116] => { put_handler( msg ); } 
             [103u8,101,116] => { get_handler( msg ); }
             [100u8,101,108] => { del_handler( msg ); }
             [99,111,109]    => { com_handler( msg ); }
             _               => { null_handler(msg ); }
        }
        }
        let data_parse = data::data::read(msg.clone());

        match data_parse {
            Ok(ok)   => {println!("{}", ok)},
            Err(err) => {panic!("{}", err)},
            // _ => { panic!(" unknown error!")},
        };

        let msg= msg.to_vec();
        if  utils::startswith(msg.clone(), "get".to_string().into_bytes()){
            println!("{}", "get ~~~~");

        }
        else if utils::startswith (msg.clone(), "put".to_string().into_bytes()){
            println!("{}", "put")
        }
        else {
            println!(" not supported! method");
        }
	}
}

pub fn feeder(uri: &str) {
	let mut socket = Socket::new(Protocol::Push).unwrap();
    socket.connect(uri).unwrap();
    // let mut endpoint = socket.connect(uri).unwrap();
    loop {
        let mut z = data::data::write();
        let mut z = z.as_slice();
	    socket.write(z);
	    // socket.write(b"message in a bottle");
        // socket.write(b"get helloworld").unwrap();
	    // socket.write(z);
    }
	// endpoint.shutdown();
	// drop(socket)
}

fn main() {
	let args = std::os::args();

	if args.len() < 3 {
		println!("Usage: pipeline worker addr:port , pipeline feeder");
        println!("like pipe worker tcp://127.0.0.1:3000 ")
		return
	}

    let uri = args[2].as_slice();
	if args[1].as_slice() == "worker".as_slice() {
	    worker(uri );
	}
	else if args[1].as_slice() == "feeder".as_slice() {
	    feeder(uri);
	}
}

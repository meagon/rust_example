
extern crate nanomsg;
// use nanomsg::result::NanoResult; 
use nanomsg::{Socket, Protocol};


fn server_init()  {
        let (tx, rx) = channel();
        let mut socket = Socket::new(Protocol::Pair).unwrap();
        let mut endpoint = socket.bind("tcp://127.0.0.1:3000").unwrap();

        spawn(proc() {
           loop {
                let msg = socket.read_to_end().unwrap();
                if msg.len() > 3u {
                    match String::from_utf8(msg[0..3].clone().to_vec()){
                        Ok(method) => {
                            match method.as_slice() {
                                "put" => handler::put(msg, &socket, &db),
                                "get" => handler::get(msg, &socket, &db),
                                "del" => handler::del(msg, &socket, &db),
                                "com" => handler::com(msg, &socket, &db),
                                 _    => {
                                        handler::error(format!("method {} not implement", method), &socket);
                                 },
                            },
                        }
                        Err(e) => handler::error(format!("method {} parse error!", msg[0..3]), &socket), 
                    };
                }else { handler::error("msg len less than than 3", &socket);}
           }
      });
}     


fn main() {
        server_init();
}


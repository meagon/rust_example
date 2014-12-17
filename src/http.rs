

#![feature(slicing_syntax)]
#![feature(phase)]
#![feature(phase)]
#[phase(plugin, link)]


extern crate tiny_http;
// extern crate hash;

extern crate log;

use std::os;
use std::sync::Arc;
use std::sync::atomic::AtomicOption;
use std::ascii::AsciiStr;
use std::rc::Rc;

mod search;
mod http_handle;

fn get_boundary(headers: &[tiny_http::Header]) -> Vec<u8> {
    let mut multipart: Vec<u8> = vec!();
    for i in headers.iter() {
        if format!("{}" , i . field) == "Content-Type".to_string() {
            for j in i.value.iter() {
                multipart.push(j.as_byte());
            }
        }
    }

    let input = String::from_utf8_lossy(multipart.as_slice());
    let mut elems = input.splitn(1, ';');
    let value = elems.next();
    println!("boundary is {}" , value.unwrap() [ 10 .. ]);
    // 神奇的数字,为神马是10 呢?  "boundary :".len() = 10;
    return value.unwrap()[10..].to_string().into_bytes(); 
}


fn main() {
    let server =
        Arc::new(tiny_http::ServerBuilder::new().with_port(2000).build().unwrap());
        //Arc::new(tiny_http::ServerBuilder::new().with_port(2000).build().unwrap());
    println!("Now listening on port 3000");

    for _ in range(0, os::num_cpus()) {
        let server = server.clone();
        spawn(move ||
              {
              for mut rq in server.incoming_requests() {
                  match rq.get_method().clone().into_string().as_slice() {
                      // will add "head" method to query file if exists;
                    "GET"  =>  { http_handle::get(&rq);  },
                    "POST" =>  { http_handle::post(&rq); },
                    "PUT"  =>  { http_handle::put(&rq);  },
                    _                  =>  { http_handle::error(&rq ,"not support method");},
                  }
              }
        });
    }
}


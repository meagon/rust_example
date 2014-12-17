

#![feature(slicing_syntax)]
#![feature(phase)]


extern crate tiny_http;
extern crate hash;
extern crate log;

use std::os;
use std::sync::Arc;
use std::ascii::AsciiStr;

use search;
use parse;


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
    return value.unwrap()[10..].to_string().into_bytes(); 
}


fn main() {
    let server =
        Arc::new(tiny_http::ServerBuilder::new().with_port(2000).build().unwrap());
    debug!("Now listening on port 2000");

    for _ in range(0, os::num_cpus()) {
        let server = server.clone();

        spawn(
              proc() {
              for mut rq in server.incoming_requests() {
                  match rq.get_method().clone().into_string() {
                      // will add "head" method to query file if exists;
                    "GET".to_string()  =>  { http_handle::get(rq);  },
                    "POST".to_string() =>  { http_handle::post(rq); },
                    "PUT".to_string()  =>  { http_handle::put(rq);  },
                    _                  =>  { http_handle::error("not support method");},
                  }
              }
           });

    }
}


                  let headers = rq.get_headers().clone();
                  let boundary = get_boundary(headers);
                  println!("{}" , headers);



                  if rq.get_method().clone().into_string() ==
                         "GET".to_string() {
                      debug!("{}" , rq . get_url (  ));
                      let response =
                          tiny_http::Response::from_string("req GET hello world".to_string());
                      rq.respond(response);
                  } else if rq.get_method().clone().into_string() ==
                                "POST".to_string() {
                      {
                          let content = rq.as_reader().read_to_end().unwrap();
                          {
                              parse::parse::upload(&content, &boundary, "upload");
                              debug!("body length = {}" , rq . get_body_length().unwrap())
                          }
                      }
                      let response =
                          tiny_http::Response::from_string("ok read".to_string());
                      rq.respond(response);
                  } else { }
              } })
    }
}

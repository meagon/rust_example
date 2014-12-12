
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
    //for i in headers.iter() {
    let mut multipart: Vec<u8> = vec!();
    for i in headers.iter() {
        // if i.field.as_byte() == "Content-Type".as_byte() {
        if format!("{}" , i . field) == "Content-Type".to_string() {
            // let mut multipart = i.value.iter().map(|c| *c.into_bytes()).collect(); //format!("{}",i.value);
            for j in i.value.iter() {
                // debug!( "{}", *j.as_byte());
                multipart.push(j.as_byte());
            }
        }
    }

    let input = String::from_utf8_lossy(multipart.as_slice());
    let mut elems = input.splitn(1, ';');
    let value = elems.next();
    println!("boundary is {}" , value.unwrap (  ) [ 10 .. ]);
    return value.unwrap()[10..].to_string().into_bytes(); //to_vec();
}



fn main() {
    let server =
        Arc::new(tiny_http::ServerBuilder::new().with_port(2000).build().unwrap());
    debug!("Now listening on port 2000");

    for _ in range(0, os::num_cpus()) {
        let server = server.clone();

        spawn(
              // let content_s = String::from_utf8_lossy(content.as_slice()); //.into_bytes();
              // debug!("{}", content_s);

              // debug!("length = {}", content_s.len());
              // hash::cal_md5(content_s);
              //debug!("{}" ,content );
              // let  content =  z.into_reader().read_to_string().unwrap();
              //  debug!("{}", z.into_reader().read_to_string().unwrap());
              proc() {
              for mut rq in server.incoming_requests() {
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
                              parse::parse::upload(&content, &boundary,
                                                   "upload");
                              debug!("body length = {}" , rq . get_body_length
                                     (  ) . unwrap (  ))
                              debug!("+++++++++++++++++++++++++++++++++++++++++++");
                          }
                      }
                      let response =
                          tiny_http::Response::from_string("ok read".to_string());
                      rq.respond(response);
                  } else { }
              } })
    }
}

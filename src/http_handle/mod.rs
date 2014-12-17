

//mod http_handle;

extern crate tiny_http;
extern crate data;
//extern crate client;

mod client;
mod parse;
//mod http_util;
//pub use client;


pub fn get(rq :&tiny_http::Request){
   
   let path = rq.get_url();
   println!("{}", path);
   let data = "hello".to_string().into_bytes();
   let response = tiny_http::Response::from_data(data);
}


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


pub fn post(rq :&tiny_http::Request){

    let headers = rq.get_headers().clone();
    let boundary = get_boundary(headers);
    let content = rq.as_reader().read_to_end().unwrap(); 
    
    let mut input_objs = parse::parse::body_parse(&content, &boundary);
    let replays : Vec<Vec<u8>>;
    for i in input_objs.iter(){
        let obj = i.clone();
        let obj_data : Vec<u8> = obj.ret;
        let filename: String = 
            match obj.header_map.get("filename"){
                Some(name) => { name.trim_chars('"').to_string() },
                None       => { "nil".to_string() },
            };
        let store_data :Vec<u8> = data::data::pack(filename, obj_data);
        let gate_client = client::gateway::new("tcp://127.0.0.1:2000");
        let replay : Vec<u8>  = gate_client.send( store_data);
        replays.push(replay);
    }
    let replay_string :Vec<String> ;
    for &i in replays.iter(){
        replay_string.push(String::from_utf8(i).unwrap());
    };
    replay_string.connect(",").to_string();
    let response = tiny_http::Response::from_string( replay_string[0] ); 
    rq.respond(response);  
}

pub fn put(rq: &tiny_http::Request){
    
    let response = tiny_http::Response::from_string("not support put now".to_string());
    rq.respond(response);
}

pub fn error(rq: &tiny_http::Request, info :&str){
    let response = tiny_http::Response::from_string(info.to_string());
    rq.respond(response);
}



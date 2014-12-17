
#![feature(slicing_syntax)]
#![feature(phase)]

#[phase(plugin, link)]
extern crate log;
extern crate regex;


#[deriving(Clone)]
enum RStatus { FoundNR, FoundLF, Content, Boundary_end, Nothing, }
pub mod parse {
    #![feature(phase)]
    #[phase(plugin, link)]
    extern crate log;
    extern crate regex;
    #[phase(plugin)]
    extern crate regex_macros;
    use search;
    use std::collections::HashMap;
    use std::option;

    #[deriving(Clone)]
    pub struct Input_obj {
        header_map: HashMap<String, String>,
        ret: Vec<u8>,
    }
    fn index_of(s: &Vec<u8>, t: &Vec<u8>) -> Vec<uint> {
        let mut ret = search::search::byyel(s, t, 0u);
        return ret;
    }
    pub fn body_parse(body: &Vec<u8>, boundary: &Vec<u8>) -> Vec<Input_obj> {
        let mut count = 0u;
        let mut headers: Vec<Input_obj> = Vec::new();

        let position_array: Vec<uint> = index_of(body, boundary);
        let boundary_len = boundary.len();
        let mut last: uint = 0;
        for i in position_array.iter() {
            if count == 0u {
                count += 1;
                last = *i + boundary_len;
                continue ;
            }
            if count == position_array.len() { break ; }
            let block: &[u8] = body[last..*i];
            let mut head_info = collectHeaders(&block.to_vec());
            match collectHeaders(&block.to_vec()) {
                Some(header_info) => { headers.push(header_info); }
                None => { }
            };
            last = *i;
        }
        //println!("{}" , body [ last .. ]);
        return headers;
    }

    fn collectHeaders(buf: &Vec<u8>) -> Option<Input_obj> {
        let mut header_map: HashMap<String, String> = HashMap::new();
        let left: Vec<u8> = Vec::new();
        let CRLFCRLF = "\r\n\r\n".to_string().into_bytes();
        let CRLF = "\r\n";
        let DASH = '-';
        let mut pos: Vec<uint> = index_of(buf, &CRLFCRLF);
        debug!("position at  ==={}" , pos);
        if pos.is_empty() { return None; }
        let pos = pos[0];
        // why pos + 4, jump  CRLFCRLF length.
        let (headerVec, reset) = (buf[0..pos], buf[pos + 4..buf.len()]);
        let mut head: Vec<u8> = headerVec.clone().to_vec();
        debug!("header  {} reset: {}" , headerVec , reset);
        let mut header = String::from_utf8(head);
        let header = header.unwrap();
        let header = header.trim();

        if header.starts_with("--") || header.is_empty() { return None; }
        let headers: Vec<&str> = header.split_str(CRLF).collect();
        for &item in headers.iter() {
            if item.starts_with("--") || header.is_empty() {
            } else if item.starts_with("Content-Disposition") {
                let element: Vec<&str> = item.split(';').collect();
                for &i in element.iter() {
                    if i.contains("=") {
                        let pairs: Vec<&str> = i.splitn(1, '=').collect();
                        let k = pairs[0].trim().to_string();
                        let v = pairs[1].trim().to_string();
                        header_map.insert(k, v);
                    }
                }
            } else if item.starts_with("Content-Type") {
                let pairs: Vec<&str> = item.splitn(1, ':').collect();
                let k = pairs[0].to_string();
                let v = pairs[1].to_string();
                debug!("Content-type :{}" , v);
                header_map.insert(k, v);
            }
        }
        let mut len = reset.len();
        let mut left: Vec<u8> = Vec::new();
        if (len >= 2u && reset[len - 2..len] == vec!(13u8 , 10).as_slice()) {
            debug!("body large than 2");
            debug!("body ends {}" , reset [ len - 2 .. len ]);
            left = reset[..len - 2].to_vec();
        } else if (len > 4u) &&
                      reset[len - 4..len] ==
                          vec!(13u8 , 10 , 45 , 45).as_slice() {
            left = reset[..len - 4].to_vec();
        } else { left = reset.to_vec(); }
        let input_instance = Input_obj{header_map: header_map, ret: left,};
        return Some(input_instance);
    }

    pub fn upload(body: &Vec<u8>, boundary: &Vec<u8> ) {
        let mut z = body_parse(body, boundary);
        for i in z.iter() {
            let mut input = i.clone();
            let mut input = input;

            let content: Vec<u8> = input.ret;
            let filename: String =
                match input.header_map.get("filename") {
                    Some(name) => { name.trim_chars('"').to_string() }
                    None => { "nil".to_string() }
                };
            if filename != "nil".to_string() {
                debug!("content is {}" , content);
                save_file(filename.as_slice(), content.as_slice());
           }
        }
    }


    fn save_file(file_name: &str, content: &[u8]) {
        use std::io::{File, Open, ReadWrite};
        use std::io::fs::PathExtensions;
        info!("will save filename {}" , file_name);
        info!("will save file {} " , content [ 0 .. 2 ]);
        let mut path = Path::new(file_name);
        let mut file =
            match File::open_mode(&path, Open, ReadWrite) {
                Ok(f) => f,
                Err(e) => { panic!("error"); }
            };
        match file.write(content) {
            Ok(f) => { debug!("ok write file"); }
            Err(e) => { debug!("error write file"); }
        }
        match file.flush() {
            Ok(f) => { debug!("flush file ok"); }
            Err(e) => { debug!("flush file error"); }
        }
    }
}

fn main() {
    use std::io::File;
    let path = Path::new("./body");
    let mut file = File::open(&path);
    let body = file.read_to_end().unwrap(); // expect("read body file");
    let boundary =
        "------------------------------77de85f2892a".to_string().into_bytes().to_vec();
    parse::body_parse(&body, &boundary);
}

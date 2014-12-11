
#![feature(slicing_syntax)]
#![feature(phase)]

#[phase(plugin, link)]
extern crate log;
extern crate regex;

mod search;

#[deriving(Clone)]
enum RStatus { FoundNR, FoundLF, Content, Boundary_end, Nothing, }
mod parse {
    #![feature(phase)]
    #[phase(plugin, link)]
    extern crate log;
    extern crate regex;
    #[phase(plugin)]
    extern crate regex_macros;
    use search;
    use std::io::fs::PathExtensions;
    use std::io::File;
    use std::collections::HashMap;
    use std::option;
    pub struct Input_obj {
        header_map: HashMap<String, String>,
        ret: Vec<u8>,
    }
    fn index_of(s: &Vec<u8>, t: &Vec<u8>) -> Vec<uint> {
        let mut ret = search::search::byyel(s, t, 0u);
        return ret;
    }
    pub fn body_parse(body: &Vec<u8>, boundary: &Vec<u8>)
     -> Vec<Option<Input_obj>> {
        let mut it = body.iter();
        let mut boundary = boundary.clone();
        let mut boundary_str = String::from_utf8(boundary).unwrap();
        let mut boundary_str = boundary_str.as_slice();
        let re = regex::Regex::new(boundary_str).unwrap();
        let mut text = String::from_utf8_lossy(body.as_slice()).into_owned();
        let mut previous: uint = 0;
        let mut begin: uint;
        let mut end: uint;
        let mut count = 0u;
        let mut headers: Vec<Option<Input_obj>> = Vec::new();
        for pos in re.find_iter(text.as_slice()) {
            if count == 0u { count += 1; continue ; }
            debug!("{}" , pos);
            let (begin, end) = pos;
            let block = body[previous..begin];
            debug!("at block ###");
            debug!("{}" , String :: from_utf8 ( block . to_vec (  ) ));
            debug!("at block ###");
            let mut head_info = collectHeaders(&block.to_vec());
            headers.push(head_info);
            previous = end;
        }
        return headers;
    }
    fn collectHeaders(buf: &Vec<u8>) -> Option<Input_obj> {
        debug!("collect .. begin..");
        let mut header_map: HashMap<String, String> = HashMap::new();
        let left: Vec<u8> = Vec::new();
        let CRLFCRLF = "\r\n\r\n".to_string().into_bytes();
        debug!("{}" , CRLFCRLF);
        let CRLF = "\r\n";
        let mut pos = index_of(buf, &CRLFCRLF);
        debug!("position at  ==={}" , pos);
        if pos.is_empty() { return None; }
        let mut pos = pos[0];
        let (headerVec, reset) = (buf[0..pos], buf[pos + 4..buf.len()]);
        let mut head = headerVec.clone().to_vec();
        debug!("header  {} reset: {}" , headerVec , reset);
        let mut header = String::from_utf8(head);
        let mut header = header.unwrap();
        let mut header = header.trim();
        let headers: Vec<&str> = header.split_str(CRLF).collect();
        if header.starts_with("--") || header.is_empty() { }
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
                println!("Content-type :{}" , v);
                header_map.insert("type".to_string(), v);
            }
        }
        let mut len = reset.len();
        let mut left: Vec<u8> = Vec::new();
        if (len >= 2u) {
            debug!("body large than 2");
            if (reset[len - 2..len] == vec!(13u8 , 10).as_slice()) {
                debug!("body ends {}" , reset [ len - 2 .. len ]);
                left = reset[..len - 2].to_vec();
            } else { let left = reset; }
        } else { left = reset.to_vec(); }
        let input_instance = Input_obj{header_map: header_map, ret: left,};
        return Some(input_instance);
    }
}

fn main() {
    use std::io::File;
    let mut path = Path::new("./body");
    let mut file = File::open(&path);
    let body = file.read_to_end().expect("read body file");
    let re =
        "------------------------------77de85f2892a".to_string().into_bytes().to_vec();
    parse::body_parse(&body, &re);
}

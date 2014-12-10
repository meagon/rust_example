


#![feature(slicing_syntax)]
#![feature(phase)]
#[phase(plugin, link)] extern crate log;


extern crate regex;
#[phase(plugin)] extern crate regex_macros;



use std::io::fs::PathExtensions;
use std::io::File;
use std::collections::HashMap;

mod search;
// ------------------------------77de85f2892a^M
// Content-Disposition: form-data; name="blob"; filename="ff.rs"^M
// Content-Type: application/octet-stream^M
// ^M
//

#[deriving(Clone)]
enum   RStatus{
    FoundNR,   // \r
    FoundLF,    // \n
    Content,    // 'C'
    Boundary_end,  // boundary 的最后一个字符
    Nothing,
}


struct Input_obj {
    header_map : HashMap<String,String>,
    ret: Vec<u8>,
};




fn indexOf( s: &Vec<u8>, t:&Vec<u8>) -> Vec<uint> {
    let mut ret = search::search::byyel(s, t, 0u);
    return ret;
}

fn main(){
    let mut path = Path::new("./body");
    let mut file =  File::open(&path);
    let body = file.read_to_end().unwrap();
    // println!("{}", body);
    let mut it = body.iter();
    let mut  state : RStatus = RStatus::Nothing;
    let mut count = 0u;
    let mut LR : Vec<uint> = Vec::new();
    let mut z :Vec<u8> = Vec::new();
    let mut CRLFCRLF : Vec<uint> = Vec::new();
    let mut cr_position  : Vec<uint> = Vec::new();
    let re = regex!(r"------------------------------77de85f2892a");
    let mut text = String::from_utf8_lossy(body.as_slice()).into_owned();
    //let mut text = text.as_slice().into_string();
    let mut previous :uint = 0;
    let mut begin :uint;
    let mut end :uint;
    for pos in re.find_iter( text.as_slice() )  { //          text.as_slice()) {
            println!("{}", pos);
            let (begin, end) = pos;
            let block = body[previous..begin];
            println!("at block ###");
            println!("{}", String::from_utf8(block.to_vec()));
            println!("at block ###");
            previous = end;
            //println!("{}", [begin..end])
    }
}






/*
    fn collectHeaders ( buf :&Vec<u8>) ->  Options<>{
        let header_map = HashMap<String, String>;
        let left :Vec<u8> = Vec::new();
        let CRLFCRLF = "\r\n\r\n".as_slice().to_vec();
        let pos = IndexOf(buf, &CRLFCRLF),
        let (headerBytes, rest) = (buf[0..pos], buf[pos+4 ..]);
        let header = String::from_utf8( headerBytes).unwrap().trim();
        let headers = header.split(header, &CRCF);
        if header.starts_with("--") || header.is_empty() {
            // return None;
        }
        for item in headers.iter(){
            if item.startswith("Content-Disposition") {
                element = line.splitn(";");
                for i in element.iter(){
                    if  i.contains("="){
                        pairs = i.splitn(1,"=");    
                        k = pairs.next().unwrap();
                        v = pairs.next().unwrap();
                        header_map.insert(k,v);
                    }
                }
            }
            if item.starts_with("Content-Type"){
                pairs = i.splitn(1, ":");
                k = pairs.next().unwrap();
                v = pairs.next().unwrap();
                header_map.insert("type",v);
            }
        }
        let left = reset.drop(CRLF);

    let input_instance = Input_obj {
        header_map: header_map,
        ret : left;
    }
    return Some(header,left);
}
            

        
        }
        else {

            let mut params = line.split(1, ":" );
            let mut field =  params.next().unwrap();
            let mut value = params.next().unwrap();
        }
        let left = reset.drop(CRLFCRLF.length);
        return Some(header, left);
    }
    
fn uselsee(){

    // Option(buffer).map(b => b.splitAt(b.indexOfSlice(CRLFCRLF))).get


    let mut boundary = "------------------------------77de85f2892a".to_string().as_bytes().to_vec();
    let mut index = indexOf(&body,&boundary);
    println!("index equal = {}",index);

    loop {
        match it.next(){
            Some(&x) => {
                z.push(x);
                if x == 10u8 {
                    state = RStatus::FoundNR;
                    //if body[count-3..count] == vec![13u8
                    if body[count-1] == 13u8 {
                        println!("found element should be [13 10 13 10] {} ", body[count-3..count+1]);
                        if body[count-3..count+1] == vec![13u8,10,13,10].as_slice()  {
                            zz.push(z.clone());
                            z.clear();
                            CRLFCRLF.push(count);
                            cr_position.push(count);
                        }
                    }
                }
                count +=1;
            }
            None => {println!("the end"); break}
        }
    }
    println!("{}  ",  cr_position);
    println!("LR   begin  :{}", LR);
    println!("=============================");
    for i in zz.iter(){
        if  ( i.len() >=7 ) {
            let mut line = String::from_utf8(i[0..6].to_vec()).unwrap();     //Content
            if line.starts_with("--"){
                println!(" line starts with  --{}", line);
            }
            
        }
        println!("{}", "==============================================eleleee");
        println!("{}",i);
        println!("{}", String::from_utf8(i.clone()).unwrap());
    }

    println!("llllllllllllllllllllllllllllllllllllllll")



    

    struct Input_obj {name: String, value : String, input_type: String, ret: Vec<u8>};
    let mut Stage_end : bool = false;
    let mut Final_end : bool = false;
    let mut previous = 0u;

    let mut name : String = "".to_string();
    let mut value: String = "".to_string();
    let mut input_type: String = "plain/text".to_string();   //default
    let mut ret : Vec<u8> = Vec::new();
    
    let mut boundary = "------------------------------77de85f2892a".to_string();
    let mut boundary_end = "------------------------------77de85f2892a--".to_string();

    let mut Forms  = Vec::new();
    // for i in CRLFCRLF.iter() {
    for line in zz.iter() {
        println!("at lopping===");
        //let mut line = String::from_utf8(body[previous..i-1].to_vec()).unwrap() ;//.unwrap();
        //debug!("{}", line.as_bytes());
        println!("{}", line);
        println!("{}", String::from_utf8(line.clone()));
        if line.starts_with("--".to_string().as_bytes()) {
            if line.starts_with(boundary.as_bytes()) && !line.starts_with( boundary_end.as_bytes() ){
                Stage_end = true;
                //break;
            }else {
                Final_end = true;
                //break;
            }
        }
        else if line.is_empty() {
            //println!("found line startswith : {}", line);
        }else if line.starts_with("Content".to_string().as_bytes()){
            let mut params = line.splitn(1, |x| x== &58u8 );
            // let mut (field, value) = (params.next(), params.next())
            let mut field =  params.next().unwrap();
            let mut value = params.next().unwrap();

            //let (field, value) = match (field, value) {
            //    (Some(f), Some(v)) => (f, v), 
            //    _ => (None,None)
            //};

            if field.starts_with("Content-disposition".to_string().as_bytes()){
                println!("================== Content-disposition  =====================");
                println!("{}", value);
                /*
                let mut input_dis = value.split(";");
                loop {
                    match input_dis.next().unwrap() {
                        Some(f) => {
                            let mut pair = f.splitn(1,"=");
                            let mut k = pair.next().unwrap();
                            if k == "name" {
                                name = pair.next.unwrap();
                            }
                        },
                        _ => { break;},
                    }
                }
                */
            }
            else if field.starts_with("Content-Type".to_string().as_bytes()){
                input_type = value.to_string();
            }
            else {
            }
            debug!("header field:{} value : {}", field, value);
        }else {
            ret.push_all(line.as_slice());
            // println!("this is file body: {} ", line);
        }
        if Stage_end {
            // struct input_obj {name: &'static str, value : &'static str, itype: &'static str, ret: Vec<u8>};
            let mut input_name = name.clone();
            let mut input_value = value.clone();
            let mut input_type_instance = input_type.clone();
            // let mut ret = ret;
            let input_instance = Input_obj { name : input_name, value : input_value, input_type: input_type_instance, ret :ret.clone() };
            Forms.push(input_instance);
            ret.clear();
        }
        
        if Final_end{
        //    break;
        }
            //previous = i+1;
    }

    println!("{}", Forms.len());
    for i in Forms.iter(){

        println!("  form name {}", i.name);
        println!(" form value {}", i.value);


    }
}
*/

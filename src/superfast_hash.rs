

fn get16bits(d) {
    u32 d[1] << 8 + u32 d[0];
}

SuperFastHash( data : &str) -> u32 {

    let mut hash : u32 = len;
    let rem :int ;

    if ( len <= 0 || data.is_empty() ) return 0u32;

    rem = len & 3;
    len >>=2 ;

    loop {
        if len <= 0 {
            break;
        }
        hash += get16bits(data);
        tmp   = (get16bits(data+2) <<11) ^hash;
        hash  = (hash << 16) ^tmp;
        data += 2*4;
        hash += hash >> 11;
        len -=1;
    }

    match (rem) {
        3 => {
            hash += get16bits(data);
            hash ^= hash << 16;
            hash ^= ((signed char) data[4] << 18;
            hash += hash >> 11; 
        }
    




}

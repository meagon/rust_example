




fn fastsearch(s:Vec<u8>, p:Vec<u8>, maxcount: u8, int mode) -> Option<uint> {

    let FAST_COUNT =0i;
    let FAST_SEARCH = 1i;
    let FAST_RESEARCH = 2i;
    let mut mask: uint;
    let mut skip = 0u16;
    let mut count = 0u16;
    let mut i :uint;
    let mut j :uint;
    let mut mlast :uint;
    let mut w :uint;

    let mut n = s.len();
    let mut m = p.len();
    w = n - m;
    
    let mut position = Vec::new();
    if ( w < 0 || (mode == FAST_COUNT &&  maxcount == 0u8))
        return None;
    if  m <= 0u
        return None;
    let mut idx = 0u;
    if m = 1u {
       if (mode == FAST_COUNT){
           let mut idx = 0u;
            for i in s.iter(){
                if *i== p[0]{
                    position.push(idx);
                    let count = count+1;
                    if (count == maxcount){
                        return maxcount;
                    }
                }
                let idx = idx+1;
            }
       }
       else if (mode == FAST_SEARCH) {
            for i in s.iter(){
                if *i == p[0]
                    position.push(idx);
                    return i;
                let idx = idx+1;
            }
       }
       else{
            for i in range(0, n){
                let mut nr = n-i;
                if s[nr] == p[0]
                    position.push(nr);
                    return nr;
            }
       }
    }

    let mlast = m-1;
    let skip = mlast -1;
    let mask = 0;
    
    if (mode != FAST_RESEARCH){
        for i in range(mlast){
            STRINGLIB_BLOOM_ADD!(mask, p[i]);
            if (p[i] == p[malst]){
                let skip = mlast -i -1;
            }
        }
        STRINGLIB_BLOOM_ADD(mask, p[mlast]);
        for i in range(w){
            if ( s[i+m-1] == p[m-1]){
                for j in range(mlast){
                    if (s[i+j] != p[j])
                        break;
                }
                if (j = mlast){
                    if (mode!= FAST_COUNT){
                        return i;
                    }
                    let count = count+1;
                    if (count == maxcount){
                        return maxcount;
                    }
                    continue;
                }

                if (!STRINGLIB_BLOOM!(mask,s[i+m])){
                    let i = i+m;
                }
                else{
                    let i = i+skip;
                }
            }
            else{
                if (!STRINGLIB_BLOOM(mask, s[i+m])){
                    let i = i+m;
                }
            }
        }
    }
    // FAST_RESEARCH
    else {
        STRINGLIB_BLOOM_ADD!(mask, p[0]);
        for r in range(0, mlast){
            let mut i = mlast-r;
            STRINGLIB_BLOOM_ADD!(mask,p[i]);
            if (p[i] == p[0])
                skip = i-1;
        }

        for r in  range(0, w){
            let mut i = w - i;
            if (s[i] == p[0]){
                let mut j;
                for rr in range(0,mlast)
                    j = mlast -rr;
                    if (s[i+j] != p[j])
                        break;
                if ( j == 0u ){
                    return i;
                }
                if (i > 0 & !STRINGLIB_BLOOM(mask, s[i-1])
                    let i = i-m;
                else
                    let i = i -skip;
            }else{
                if (i>0 & !STRINGLIB_BLOOM(mask, s[i-1])
                    let i = i-m;
            }
        }
    }
    if (mode != FAST_COUNT){
        return -1;
    }
    return count;
}














    



#![feature(macro_rules)]

// 0UL 表示 无符号长整型 0
// 1UL 表示 无符号长整型 1


/// #define STRINGLIB_BLOOM_ADD(mask, ch) \
///    ((mask |= (1UL << ((ch) & (STRINGLIB_BLOOM_WIDTH -1)))))
/// #define STRINGLIB_BLOOM(mask, ch)     \
///     ((mask &  (1UL << ((ch) & (STRINGLIB_BLOOM_WIDTH -1)))))


macro_rules! STRING_BLOOM_ADD(
    (mask:expr, $ch:expr) => (( *mask |= (1u64 << ((ch) & (64u -1 )))) > 0 as uint)
)

macro_rules! STRINGLIB_BLOOM(
    (mask:expr, $ch:expr) => ((mask & ( 1u64 << ((ch) & (64u -1 )))) >0 as bool)
)

///  macro_rules! utf8_first_byte(
///      ($byte:expr, $width:expr) => (($byte & (0x7F >> $width)) as u32)
///  )
/// #define STRINGLIB_BLOOM_WIDTH 32



fn stringlib_bloom_add(mask :&uint, ch :&u8 ) -> uint{
        let z= *mask | 1u << ((*ch as uint & (32-1)));
        println!("at  vloom add {}", z); 
        return z;
}

fn stringlib_bloom(mask :&uint, ch :&u8) ->bool {
        let z = *mask & (1u << ((*ch as uint & (32-1))));
        println!("at bloom {}",z);
        return z > 0;
}



fn fastsearch( this :Vec<u8>, target : Vec<u8>,) -> Option<uint>{

    let FAST_COUNT = 0u;
    let FAST_SEARCH = 1u;
    let mut  FAST_RSEARCH =  2u;
    let mut mask : u64;
    let STRINGLIB_BLOOM_WIDTH =  32u;

    let maxcount =10u;
    let mode = 1u;
    let mut skip =0u;
    let mut count = 0u;
    
    let mut n = this.len();
    let mut m = target.len();
    let mut w = this.len() - target.len();

    if ( w <=0 ){
        return None;
    }

    /* look for special cases */
    if ( m <= 1 ){
        let mut i :uint;
        if ( mode == FAST_COUNT ) {
            i = 0;
            while (i <n){
                if (this[i] == target[0]){
                    let mut count = count + 1;
                    if ( count == maxcount) {
                        return Some(maxcount);
                    }
                }
                i += 1;
            }
            return Some(count);

            }else if (mode == FAST_SEARCH){
                i = 0; 
                while ( i< n){
                    if ( this[i] == target[0] ){
                        return Some(i);
                    }
                }
            }else {
                i = n-1;
                while ( i >=0 ){
                    if (this[i] == target[0]){
                        return Some(i);
                    }
                    i-=1;
                }
            }
            return None;
         }
        
        let mut mlast = m-1;
        let mut skip = mlast -1;
        let mut mask = 0;
        let mut i :uint;
        if (mode != FAST_RSEARCH) {
            i = 0;
            while (i < mlast){            
                mask=stringlib_bloom_add(&mask, &target[i]);
                if (target[i] == target[mlast]){
                    skip = mlast -i -1;
                }

                i += 1;
            }
            mask=stringlib_bloom_add(&mask, &target[mlast]);
                i =0;
                while ( i< w){
                if (this[i+m-1] == target[m-1]){
                        let mut j :uint;
                        j =0;
                        while (i < mlast){
                            if (this[i+j] !=target[j]){
                                break;
                            }
                            i += 1;
                        }
                    if ( j == mlast ) {
                        if (mode != FAST_COUNT){
                            return Some(i);
                        }
                        count +=1;
                        if (count == maxcount){
                            return Some(maxcount);
                        }
                        i = i+ mlast;
                        continue;
                    }
                    /* miss: check if next character is part of pattern*/
                    if (!stringlib_bloom(&mask, &this[i+m])){
                        i = i +m;
                    }
                    else{
                        i = i+skip;
                    }
                } else {
                    if (!stringlib_bloom(&mask, &this[i+m])){
                        i = i+m;
                    }
                }
                i+=1;
            }
        } else { /* FAST_RESEARC */
            /* create compressed boyer-moore delta 1 table */
            mask=stringlib_bloom_add(&mask, &target[0]);
            i = mlast;
            while ( i> 0){
                mask=stringlib_bloom_add(&mask,&target[i]);
                if (target[i] == target[0]){
                    skip = i -1;
                }
                i -= 1;
            }
             i= w;
             while ( i>=0 ){
                if (this[i] == target[0]){
                    /* candidate match*/
                    let mut j = mlast;
                    while ( j > 0){
                        if (this[i+j] != target[j]){
                            break;
                        }
                        j -= 1;
                    }
                    if (j == 0){
                        /* got a match!*/
                        return Some(i);
                    }
                    if ( i>0  && !stringlib_bloom(&mask, &this[i-1])){
                        let mut i = i-m;
                    }  else{
                        i = i - skip;
                    }
                }else {
                    if (i>0 && !stringlib_bloom(&mask, &this[i-1])){
                        i = i-m;
                    }
                }
            }
        }
        if (mode != FAST_COUNT){
            return None;
        }
        return Some(count);
    }


fn main(){


    let a = vec![1u8,2,3,4,45,56,6,677,78,8,9];
    let b = vec![8u8,9];
    let z = fastsearch(a,b);
    println!(" found z at {}",z);
}



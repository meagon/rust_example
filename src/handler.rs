/* msg  like  "method[sapce]data" 
 * 
 * match  method is put, then data is encoded data
 * get  lead to   data is search key,
 * if method is del, TODO 
 * if method is com, then will change sqlite3 data, 
 * may need a lock,
 * socket.write will send to client;
 */

mod handler{

    fn get(msg : &Vec<u8>, socket){
        let mut key = msg[4..].clone();
        let mut v = leveldb.get(key);
            match v ={
                Some(data) => return data.unpack;
                _ => return "not_found";
            }
        socket.write();
    }

    fn put(msg: &Vec<u8>, socket){
        let mut data = msg[4..].clone();
        storage_obj = data.unpack_it();
        k = storage_obj.get_key();
        v = storage_obj.get_value()
        data.store_in_leveldb(k ,v);
        return ok(data);
    }

    fn del(msg; &Vec<u8>, socket){
        return ok //, but not implement;    
    }
    fn com(msg: &Vec<u8>, socket){
        // change_sqlite_status;
        return ok,
    }
    
    fn  error( msg : &str) {
        return msg;
    }
}


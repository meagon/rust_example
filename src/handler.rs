/* msg  like  "method[sapce]data" 
 * 
 * match  method is put, then data is encoded data
 * get  lead to   data is search key,
 * if method is del, TODO 
 * if method is com, then will change sqlite3 data, 
 * may need a lock,
 * socket.write will send to client;
 */

#[deriving(Show)]
enum Hand { ok , error , not_found}


mod handler{
    fn get(msg : &Vec<u8>, socket :&nanomsg::Socket, ,db : &leveldb::DB)   -> Result<Version, &'static str>{
        let mut key = msg[4..].clone();
        let mut v = db.leveldb_get(key);
            match v = {
                Ok(Some(data)) =>  data;
                Ok(None)  =>  "not_found".as_bytes();
                _ => "error".as_bytes();
            }
        socket.write(v.as_slice());
        Ok(Hand::ok) 
    }

    fn put(msg: &Vec<u8>, socket: &nanomsg::Socket, db: &leveldb::DB ) -> Result<Version, &'static str>{
        let mut data = msg[4..].clone().to_vec();
        let mut data = data::unpack(data).unwrap();
        // storage_obj = data.unpack();
        // k = storage_obj.get_key();
        // v = storage_obj.get_value()
        let k = data::get_key(&data);
        let v = data::get_value(&data);
        db.leveldb_store(k ,v);
        socket.write(v);
        return Ok(Hand::ok);
    }

    fn del(msg; &Vec<u8>, socket: &nanomsg::Socket, db: &leveldb::DB) -> Result<Version, &'static str>{
        return Ok(Hand::ok); //, but not implement;    
    }
    fn com(msg: &Vec<u8>, socket: &nanomsg::Socket, db: &leveldb::DB) -> Result<Version, &'static str>{
        // change_sqlite_status;
        return Ok(Hand::ok);
    }
    
    fn  error( msg : &str,socket: nanomsg::Socket,) -> Result<Version, &'static str>{
        socket.write(msg.as_bytes());
        return Ok(hand::ok);
    }
}


extern crate "leveldb-rs" as leveldb;

use std::io::TempDir;

pub enum DBError{
    // leveldb::LevelDBError::LibraryError(String),
    // leveldb::LevelDBError::OutOfMemoryError,

    LibraryError(String),
    OutOfMemoryError,
}

pub type DBResult <T> = Result<T, DBError>;



pub fn new_temp_db(name: &str) -> leveldb::DB {
    let mut opts = leveldb::DBOptions::new().expect("error creating options");
    opts.set_create_if_missing(true);
    let tdir =
        match TempDir::new(name) {
            Ok(t) => t,
            Err(why) => panic!("Error creating temp dir :{}", why ),
        };
    match leveldb::DB::open_with_opts(tdir.path(), opts) {
        Ok(db) => db,
        Err(why) => panic!("Error create db: {}" , why),
    }
}

pub fn db_put(db : & mut leveldb::DB, key : &[u8], v: &[u8]) -> DBResult <()> {
        let ret = db.put(key, v);
        match ret {
         Ok(o) => {  return Ok(()) },
         _  => {panic!("error occur at insert db ");},
        }
}

pub fn db_get(db : & mut leveldb::DB, key : &[u8]) ->  DBResult<Option<Vec<u8>>> {
    let ret = db.get(key);
    match  ret {
        Ok( None) => { return Ok(None)},
        Ok( Some(ret)) => return {Ok(Some(ret))},
        _ => { panic!("error while get key: {}", String::from_utf8_lossy(key) ) }
    }
}



fn main( ) {
    let mut db = new_temp_db("test_01");

    let mut key = b"hello";
    let mut val = b"world";

    db.put(key, val).unwrap();
    let val =
        match db.get(key) {
            Ok(val) => val.expect("Expected to find key 'abc'"),
            Err(why) => panic!("Error getting from DB: {}" , why),
        };
    println!("{}", String::from_utf8_lossy(val.as_slice()));
}

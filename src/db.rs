
#![feature(slicing_syntax)]

extern crate "leveldb-rs" as leveldb;

use std::io::TempDir;
use std::io::fs::PathExtensions;
use std::io;
use std::io::fs;

pub enum DBError{
    LibraryError(String),
    OutOfMemoryError,
}

pub type DBResult <T> = Result<T, DBError>;

pub fn new_temp_db(name: &str) -> leveldb::DB {
    let mut opts = leveldb::DBOptions::new().expect("error creating options");
    opts.set_create_if_missing(true);
    let tdir = Path::new(name);
    fs::mkdir(&tdir, io::USER_RWX);
    
    match leveldb::DB::open_with_opts(& tdir, opts) {
        Ok(db) => db,
        Err(why) => panic!("Error create db: {}" , why),
    }
}

pub fn db_put(db : & mut leveldb::DB, key : &[u8], v: &[u8]) -> DBResult <Option<String>> {
        let ret = db.put(key, v);
        match ret {
         Ok(o) => {  return Ok(("ok".to_string())) },
         _  => {Return Ok(Some("error occur at insert db "));},
        }
}

pub fn db_get(db : & mut leveldb::DB, key : &[u8]) ->  DBResult<Option<Vec<u8>>> {
    let ret = db.get(key);
    match  ret {
        Ok( None) => { return Ok(None)},
        Ok(Some(ret)) => return {Ok(Some(ret))},
        _ => { return Ok(Some("error while get key: {}", String::from_utf8_lossy(key) )) }
    }
}

/*
pub fn db_iter(db :& mut leveldb::DB) -> DBResult<Option<Vec<u8>>>{
    let ret =  db.iter();
    match ret {
        Ok(ok) =>  { return ok; },
        _ => { panic!("error while iter keys") },
    }
}

*/

/*
 *
 * TODO --
open, 
open, async
close, 
get,
/* async  get */

/* */
put
( db: db_ref, key : bytes, val: bytes,  ..write_options)

/*del an item */
delete
(db: db_ref , Key :bytes , opts)

/*  sync  mem info to diskes  */
write

async_write()
(CallerRef, db_ref, updates, opts)

/*  async write */
fold, 

%% how many and what keys does a fold contains
fold_keys
status
/* status */
destroy
/* repair */
repair,
/* */
is_empty
/* */

iterator,
/*  async iter */

/* for different db exchange db items */
iterator_move,

/* iterator over*/
iterator_close

*/


fn get_path() -> String  {
        let  default_db_path = "./test_dir/"; //as_byte(); //to_string().as_slice();
        let mut db_path = default_db_path;
        let mut dbpath =  std::os::getenv("DBPATH");
        let mut db  =  match dbpath  {
            Some( path) => { path }
            None =>  default_db_path.to_string(),
        };
        return  String::from_utf8(db.into_bytes()).unwrap();
}


fn main( ) {

    let  default_db_path = "./test_dir/";
    let mut dbpath =  std::os::getenv("DBPATH");
    let mut db  =  match dbpath  {
        Some( path) => { path }
        None =>  default_db_path.to_string(),
    }; 
    let db_path = db.as_slice();

    let mut db = new_temp_db( db_path );
    let mut key = b"hello";
    let mut val = b"world";

    db_put(& mut db ,key, val);
    let val =
        match db_get(& mut db, key) {
            Ok(val) => val.expect("Expected to find key 'abc'"),
            Err(why) => panic!("Error getting from DB: "),
        };
    println!("{}", String::from_utf8_lossy(val.as_slice()));
}

struct Storage {
    path : String;
    db   : leveldb::DB;
}

pub fn new_storage(path : String ) -> {
    let mut opts = leveldb::DBOptions::new().expect("error creating options");
    opts.set_create_if_missing(true);
    let tdir = Path::new(path.as_slice());
    fs::mkdir(&self.path, io::USER_RWX);
    match leveldb::DB::open_with_opts(& tdir, opts) {
              Ok(db) => db, 
              Err(why) => panic!("Error create db: {}" , why),
    };

    Ok{
        (Storage{
            db : db,
            path: String;
        })
    }
}

impl Storage {
    pub fn get_key( key : &[u8]) -> DBResult<Option<Vec<u8>>> {
        let ret = self.db.get(key);
        match ret {
            Ok(None) => { return Ok(None) },
            Ok(Some(ret)) => return { Ok(Some(ret))},
            _ => { panic!("error while get key: {}",
                          String::from_utf8_lossy(key));
            }
        }
    }

    pub fn set_kv(k: &[u8], v:&[u8]) -> DBResult<Option<String>> {
        let ret = self.db.put(key,v);
        match ret {
            Ok(o) => { return Ok(("ok".to_string())) },
            _     => { panic!("error occur at store key: {}",
                String::from_utf8_lossy(key));
            }
        }
    }
}


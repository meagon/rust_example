@0xf655c1c915b154d4;

struct  RecordInfo{ 
    crc @0  :Text;  #  checksums;
    ops @1  :Text;  #  INSERT ,UPDATE, DELETE;
    stmt @2 :Text;  #  query sequence
}

struct  Records {
    name @0 :List( RecordInfo );  # for lists of record
}


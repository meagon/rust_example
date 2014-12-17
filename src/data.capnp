@0xac0c661a42dc87fc;

struct  DelFlag {
   del @0 : Bool = false ; # [{0,not_deleted}, {1,deleted}] 
}
struct  DataObject{ 
    key @0   :Text;
    data @1  :Data;                      # file;
    struct Metadata {
        origname   @0     :Text;
        timestamp  @1     :Text;         # data size
        meta       @2     :Data;         # custom-metadata
        checksum   @3     :UInt32 = 0;   # checksum (MD5 > hex-to-integer)
        ringHash   @4     :UInt32;       # RING's Hash(CRC32) when write an object.
        ver        @6     :UInt8 = 0;    # version number
        delFlag    @5     :DelFlag;      # delete flag 
    }
}
struct Bulk {
    name @0 :Text;
}



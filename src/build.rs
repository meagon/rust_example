extern crate capnpc;

fn main() {
    ::capnpc::compile(Path::new("."),
                      &[Path::new("data.capnp")]).unwrap(); //.unwrap();
}

use std::io;

extern crate bongodb;

fn main() -> io::Result<()> {
    bongodb::server::run()
}

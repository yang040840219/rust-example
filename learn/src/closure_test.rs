#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]

use std::fs::File;
use std::io::ErrorKind;

#[test]
fn read_file() {
    let path = "hello.txt";
    let f = File::open(path).unwrap_or_else(|e1| {
        if e1.kind() == ErrorKind::NotFound {
            return File::create(path).unwrap_or_else(|e2| {
                panic!("create error:{:?},{}", e2, path) ;
            }) ;
        } else {
            panic!("read error: {}", path)
        }
    }) ;
}
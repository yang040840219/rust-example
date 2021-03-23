#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
use std::io;
use std::io::Read;
use std::fs::File;

#[test]
fn test_give_charge() {
    fn give_charge(gift: Option<&str>) {
        match gift {
            Some("snake") => println!("throwing"),
            Some(inner) => println!("nice: {}", inner),
            None => print!("no")
        }
        let inside = gift.unwrap();
        println!("inside:{}", inside);
    }

    let food = Some("chicken");
    give_charge(food);

    give_charge(None)
}


#[test]
fn test_give_result() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("open file error:{:?}", error)
        }
    };
    println!("{:?}", f)
}


#[test]
fn test_except() {
    // except 和 unwrap 类似
    // except 错误信息可以指定 , unwrap 是直接使用 panic! 的信息
    let f = File::open("hello.txt").expect("failed open file");
}

#[test]
fn test_propagating() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let path = "hello.txt";
        let f = File::open(path);
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e)
        };

        let mut s = String::new();

        return match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e)
        };

        // 使用 ? 想当与对 match 的简写, 像return 一样
        // f.read_to_string(&mut s)? ;
        // return Ok(s);
    }

    let result = read_username_from_file();
    match result {
        Ok(s) => println!("read:{}", s),
        Err(e) => println!("read error: {:?}", e)
    }
}
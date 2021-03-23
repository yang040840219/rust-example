#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
mod io_test {
    use std::env;
    use std::fs;

    #[derive(Debug)]
    struct User { name: String }

    #[test]
    fn env_args() {
        let args: Vec<String> = env::args().collect();
        print!("args:{:?}", args);
    }

    #[test]
    fn read_file() {
        let filename = "/opt/data/poem.txt";
        let contents = fs::read_to_string(filename).expect("error read file");
        // println!("text: {}", contents);

        fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
            let q = query.to_lowercase();
            let mut results = Vec::new();

            for line in contents.lines() {
                if line.to_lowercase().contains(&q) {
                    results.push(line);
                }
            }
            return results;
        }

        let query = "dreary" ;
        let result = search(query, &contents);
        println!("result:{:?}", result)
    }

    #[test]
    fn slice_test() {

        // slice 的作用是不拥有所有权(ownership)
        fn first_word(s: &str) -> &str {
        // fn first_word(s: &String) -> &str {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i] ;
                }
            }
            return &s[..] ;
        }

        let mut s = String::from("hello word") ;
        // let r = first_word(&s) ; // r 相当于持有 s 的不可变引用
        let r = first_word(&s[..]) ;
        // s.clear() // 调用clear() 的时候还是会报错
        println!("r:{}", r) ;
    }

    #[test]
    fn iterator_test() {
        let u1 = User{name:String::from("a")} ;
        let u2 = User{name: String::from("b")} ;
        let v1 = vec![u1, u2] ;
        let it1 = v1.iter() ;
        for v in it1 {
            println!("{:?}", v) ;
        }
        // println!("{:?}", u1) ;
        println!("{:?}", v1.get(0).unwrap())
    }
}
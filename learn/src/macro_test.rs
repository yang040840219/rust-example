#[allow(dead_code, unused_variables, unused_mut)]

#[cfg(test)]
mod macro_test {
    use std::collections::HashMap;
    /*
       在语法解析之后起作用，可以实现在编译阶段检查
       生成程序的程序
    */
    macro_rules! hashmap {
    ($( $key:expr => $var:expr ),*) => {{
        let mut map = std::collections::HashMap::new();
        $( map.insert($key, $var); )*
        map
    }}
}

    #[test]
    fn hashmap_macro_test() {
        let counts = hashmap!["A" => 0, "B" => 1];
        println!("{:?}", counts)
    }

    #[test]
    fn vec_macro_test() {
        let v = vec![1, 2, 3, 4, 5];
        println!("{:?}", v);
    }

    #[test]
    fn map_test() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        println!("{:?}", map)
    }

    #[test]
    fn mut_str() {
        let mut s = String::from("hello") ;
        s.push_str(" world") ;
        println!("{}", s);
    }
}

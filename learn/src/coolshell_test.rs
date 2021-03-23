#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
mod coolshell_test {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: i32,
    }

    #[test]
    fn get_stack_person() {
        // 从函数栈上返回对象
        fn new_person() -> Person {
            let person = Person {
                name: String::from("hello"),
                age: 10,
            };
            return person;
        }

        let p = new_person();
        println!("{:?}", p);
    }

    #[test]
    fn owner_person() {
        let p = Person { name: String::from("hello"), age: 10 };
        let _name = p.name;
        println!("_name:{}, age:{}", _name, p.age);
        //println!("person:{:?}", p); // value borrowed here after partial move
        // 结构体重的成员是可以被move 掉的, 此时结构体是不完整的
    }


    #[test]
    fn compare_person_age() {
        // 传值的方式就需要交出所有权
        // 采用传引用的方式， rust 中称为借用
        fn compare(s1: &Person, s2: &Person) -> bool {
            if s1.age > s2.age {
                return true;
            } else {
                return false;
            }
        }

        let u1 = Person{name:"hello".to_string(), age: 10};
        let u2 = Person{name:"world".to_string(), age: 11};

        let r = compare(&u1, &u2);
        println!("r: {}", r);
        println!("u1:{:?}", u1);
    }

    #[test]
    fn vec_lifetime() {
        let args = vec![1,2,3,4,5] ;
        let first = args[0] ;
        let second = args[1] ;
        println!("first:{}, second:{}", first, second) ;
        println!("args:{:?}", args) ;
    }

    #[test]
    fn simple_lifetime() {
        //  C++ 中的悬垂指针， 出了 str_lifetime 方法之后， s 的生命周期已经结束了会被回收。
        //  传递出去的引用也就有问题了
        //  expected named lifetime parameter
        // fn str_lifetime() -> &str {
        //     let s = String::from("hello");
        //     return s.as_str() ;
        // }
        //
        // let m = str_lifetime();
        // println!("m:{}", m);

    }

    #[test]
    fn compare_lifetime_expire() {
        //  expected named lifetime parameter
        // 编译器不能确定返回的是s1 还是 s2,因为这些是运行时决定的.
        // 需要使用带有生命周期的引用,来告诉编译器引用值和返回值有相同的生命周期
        // fn order_string(s1: &str, s2:&str) -> (&str, &str) {
        //     if s1.len() < s2.len() {
        //         return (s2, s1) ;
        //     } else {
        //         return (s1, s2);
        //     }
        // }

        fn order_string<'a>(s1: &'a str, s2:&'a str) -> (&'a str, &'a str) {
            if s1.len() < s2.len() {
                return (s2, s1) ;
            } else {
                return (s1, s2);
            }
        }

        let str1 = String::from("hello");
        let str2 = "hello world" ;
        // order_string 使用的是字符串slice, 因为不希望order_string 获取 str1, str2 的ownership
        let (s1, s2) = order_string(str1.as_str(), str2);
        println!("s1:{}, s2:{}", s1, s2) ;
    }

}
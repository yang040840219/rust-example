#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
mod owner_test {
    use std::ops::Drop;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn it_test() {
        let a: i32 = 2;
        let b: i32 = 2;
        assert_eq!(a + b, 4);
    }

    #[test]
    fn loop_test() {
        let mut counter: i32 = 0;
        loop {
            println!("in loop: {}", counter);
            if counter == 10 {
                break;
            }
            counter += 1;
        }
    }

    #[test]
    fn fib_test() {
        fn fib(i: i32) -> i32 {
            if i == 1 || i == 2 {
                return 1;
            } else {
                return fib(i - 1) + fib(i - 2);
            }
        }

        let result = fib(4);
        println!("result:{}", result)
    }

    #[test]
    fn array_test() {
        let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
        // for e in &arr {
        //     println!("{}",e) ;
        // }
        arr[2] = 6;
        // println!("arr[2]:{}", arr[2]);

        fn print_array(array: [i32; 5]) {
            for i in 0..array.len() {
                println!("index:{}, element:{}", i, array[i])
            }
        }

        print_array(arr);
    }

    /*
     编译的时候数据的类型和大小固定，就是分配在栈上，否则分配在堆上
     作用域: {} 变量可见范围
     owner: 每个值都对应一个变量， 变量称为值的owner， 当owner 超过作用域之后就会被销毁
     移动: owner 移动
     clone: 避免 owner 移动情况，可以使用clone
     */

    #[test]
    fn owner_test() {
        let x: i32 = 1; // x 定义的时候类型固定， 因此分配在栈上
        {
            let y: i32 = 1;
            println!("x:{}", x);
            println!("y:{}", y);
        }
        println!("x:{}", x);
        {
            // 基本类型可以实现拷贝赋值操作
            let y = x;
            println!("y:{}", y)
        }
        {
            let mut s1: String = String::from("hello"); // s1的引用存储在栈上, 内容hello存储在堆上
            s1.push_str(" world");
            println!("s1:{}", s1);
            // 实际上s2 = s1 使用浅拷贝,但是出了作用域 s1, s2 都要 drop, 有问题， 因此会发送owner 移动
            let s2: String = s1;
            // s1 在此将被回收，会调用 drop 方法, 再使用会报错
            println!("s2:{}", s2);
            // println!("s1:{}", s1); // value borrowed here after move
        }
        {
            let s1: String = String::from("hello");
            let s2: String = s1.clone();
            println!("s1 clone:{}", s1);
            println!("s2 clone:{}", s2);
        }
    }

    #[test]
    fn function_owner_test() {
        fn create() -> String {
            let s = String::from("hello world");
            return s; // 所有权发生转义， 从函数内部转移到函数外部
        }

        fn consume(s: String) { // 所有权发生转移, 从函数外部转移到函数内部
            println!("s:{}", s);
        }

        let s = create();
        consume(s);
        // println!("{}", s); // value borrowed here after move
    }

    #[test]
    fn move_test() {
        #[derive(Copy, Clone)]
        struct Foo {
            data: i32
        }

        // impl Clone for Foo {
        //     fn clone(&self) -> Foo {
        //         Foo { data: self.data}
        //     }
        // }
        //
        // impl Copy for Foo {}

        let v1 = Foo { data: 1 };
        let v2 = v1;
        println!("{}", v1.data);
    }

    #[test]
    fn box_test() {
        // Box 类型 -> 指针类型  拥有所有权的指针

        #[derive(Debug)]
        struct T {
            value: i32
        }

        let p = Box::new(T { value: 1 });
        println!("p:{:?}", p);
    }

    #[test]
    fn drop_test() {
        struct D {
            value: i32
        }
        impl Drop for D {
            fn drop(&mut self) {
                println!("destruct: {}", self.value)
            }
        }

        let x = D { value: 1 };
        println!("construct 1");
        {
            let y = D { value: 2 };
            println!("construct 2");
            let z = D { value: 3 };
            println!("construct 3");
            // 局部变量定义在栈上， 先构造的后析构， 栈的策略为先进后出
        }
        println!("exit main function");
    }

    // 函数执行完成后，x 的析构函数被调用
    #[test]
    fn raii_test() {
        let path = "/opt/data/test.csv";
        let f = File::open(path);
        if f.is_err() {
            println!("file not exists");
            return;
        }
        let mut handler = f.unwrap();
        let mut content = String::new();
        let result = handler.read_to_string(&mut content);
        if result.is_err() {
            println!("read file error");
            return;
        }
        println!("content:{}", content);

        println!("result:{}", result.unwrap())

        // RAII , 无须手动关闭 file
    }

    #[test]
    fn vec_simple_test() {
        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        let one = &v[0];
        println!("{}", one);

        let two = v[0] ;
        println!("{}", two);

        // 不可变遍历,  写 v 执行完 for 之后会失去控制权，因此需要使用引用
        for i in &v {
            println!("i:{}", i);
        }
        println!("-------");
        // 可变遍历
        for i in &mut v {
            *i = *i + 1;
            println!("i:{}", i);
        }
    }

    #[test]
    fn vec_enum_test() {
        #[derive(Debug)]
        enum Context {
            Text(String),
            Float(f32),
            Int(i32),
        }

        let mut v: Vec<Context> = Vec::new();
        // push 使用的是 Vec 的可变引用
        v.push(Context::Text(String::from("hello")));
        v.push(Context::Float(1.0));
        v.push(Context::Int(2));

        for i in &v {
            println!("{:?}", i)
        }
    }

    #[test]


    #[test]
    fn message_test() {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            Change(i32, i32, i32),
        }

        impl Message {
            fn custom_print(&self) {
                match self {
                    Message::Quit => println!("quit"),
                    Message::Move { x, y } => println!("move x:{}, y:{}", x, y),
                    Message::Write(x) => println!("write:{}", x),
                    Message::Change(a, b, c) =>
                        println!("change a:{}, b:{}, c:{}", a, b, c),
                }
            }
        }

        let m = Message::Write(String::from("hello"));
        m.custom_print();
        println!("{:?}", m)
    }
}


#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]

mod pointer_test {

    use std::ops::Deref;

    /*
     智能指针拥有(ownership)其指向的数据
     Box<T>  数据在堆上， 引用在栈上， 出了作用域一起销毁。 可以用来创建递归类型， 在编译时知道占用了多少空间
    */

    #[test]
    fn box_test_1() {
        let b = Box::new(5);
        println!("b:{}", b);
    }

    #[test]
    fn box_test_2() {

        #[derive(Debug)]
        enum List {
            Cons(i32, Box<List>),
            Nil
        }

        // 使用了递归类型
        let list = List::Cons(1,
                              Box::new(List::Cons(2,
                              Box::new(List::Cons(3,
                              Box::new(List::Nil)))))) ;

        println!("list:{:?}", list)

    }

    #[test]
    fn box_test_3() {
        struct MyBox<T>(T) ;

        impl<T> Deref for MyBox<T> {
            type Target = T ;
            fn deref(&self) -> &T {
                &(self.0)
            }

            // fn new(x:T) -> MyBox<T> {
            //     return MyBox(x);
            // }

        }
        let x = 5 ;
        let y = MyBox(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn box_test_4() {
        // 获取Box所有权并销毁
        fn eat_box_i32(boxed_i32: Box<i32>) {
            println!("Destroying box that contains {}", boxed_i32);
        }

        // 此函数借用了一个 i32 类型
        fn borrow_i32(borrowed_i32: &i32) {
            println!("This int is: {}", borrowed_i32);
        }

        let boxed_i32 = Box::new(5); // heap
        let stacked_i32 = 6; // stack

        borrow_i32(&boxed_i32);
        borrow_i32(&stacked_i32);

        {
            let _ref = &boxed_i32 ;
            println!("_ref:{}", _ref);
            // eat_box_i32(boxed_i32);
        }

        eat_box_i32(boxed_i32);
    }

    #[test]
    fn ref_1() {
        #[derive(Clone, Copy)]
        struct Point { x: i32, y:i32}

        let c = "hello";

        let ref ref_c1 = c ;
        let ref_c2 = &c ;

        println!("{}", ref_c1 == ref_c2)
    }

}
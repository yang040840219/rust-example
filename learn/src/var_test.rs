#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
mod var_test {
    use std::iter::Iterator;

    #[test]
    fn tuple_test() {
        let p: (i32, i32) = (1, 2);
        let (a, b) = p;
        println!("a:{}, b:{}", a, b)
    }

    #[test]
    fn struct_test() {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 1, y: 2 };
        print!("point: x:{}, y:{}", p.x, p.y);
        // 成员名字需相同
        let Point { x, y } = p;
        print!("point: x:{}, y:{}", x, y);
    }

    #[test]
    fn tuple_struct_test() {
        struct Inches(i32);
        fn f1(value: Inches) {
            println!("value:{}", value.0)
        }
        // fn f2(value: i32) {}
        let v: i32 = 10;
        f1(Inches(v));
    }


    #[test]
    fn enum_test() {
        enum Number {
            Int(i32),
            Float(f32),
        }

        fn read_num(num: Number) {
            match num {
                Number::Int(value) => println!("Integer: {}", value),
                Number::Float(value) => println!("Float: {}", value),
            }
        }
        let n: Number = Number::Int(10);
        read_num(n);
    }

    #[test]
    fn adt_test() {
        let array = [1, 2, 3, 4, 5];
        let v: Vec<Option<&i32>> = array.iter().map(Some).collect();
        println!("{:?}", v)
    }

    #[test]
    fn ref_point() {
        // #[derive(Copy, Clone)]
        struct Foo {
            id: i32,
        }

        fn print_foo(x: &mut Foo) {
            println!("{}", x.id)
        }

        let mut f1 = Foo { id: 1 };
        let f2 = &mut f1;
        f2.id = 2;
        print_foo(f2);
        print_foo(&mut f1);
        let f3 = &f1;
    }

    #[test]
    fn ref_test() {
        let mut num = 5;
        let num_ref = &mut num;
        *num_ref = 100;
        println!("{}", num);
    }


    #[test]
    fn borrow_test() {
        fn foo(v: &mut Vec<i32>) {
            v.push(5);
        }
        let mut v = vec![];
        foo(&mut v);
        println!("{:?}", v);
    }


    #[test]
    fn memory_address() {
        let mut a = 7;
        let b = &a;

        // 引用存在，会导致原来的变量 a Frozen
        // a = 2 ;

        // 查看变量地址
        println!("{:?}", b as *const i32);

        // 查看不同类型变量占用内存
        let size = std::mem::size_of::<i32>();
        println!("{}", size);

        // 查看地址中的数值
        let borrowed = &a;
        let boxed = Box::new(a);

        println!("pointer: {}, {}", borrowed, boxed);
        println!("dereference: {}, {}", *borrowed, *boxed); // 解引用时不需要添加 *, 自动识别

        let m = borrowed + 1;
        println!("m: {}", m); // 相当于自动的解引用， 直接返回 borrowed 中的值

        let c = &a as *const i32;
        println!("{:?}", c);

        let p = 0 as *const i32;
        println!("{:p}", p);
        // 不允许显示解引用操作， 需要在unsafe 代码块中进行
        // let k = *p ;
    }

    #[test]
    fn borrow_semantics() {
        let array = vec![1, 2, 3, 4];
        // 引用传递
        fn borrow(v: &Vec<i32>) {
            // v.push(10);  // 报错
            // 打印参数占用空间的大小,在64位系统上,结果为8,表明该指针与普通裸指针的内部表示方法相同
            println!("borrow size of param: {}", std::mem::size_of::<&Vec<i32>>());
            for item in v {
                print!("{} ", item)
            }
            println!()
        }

        borrow(&array);
        println!("array[0]: {}", array[0]);

        // 值传递
        fn move_(v: Vec<i32>) {
            // 打印参数占用空间的大小,结果为24,表明实参中栈上分配的内存空间复制到了函数的形参中
            println!("move size of param: {}", std::mem::size_of::<Vec<i32>>());
            for item in v {
                print!("{} ", item)
            }
            println!()
        }

        move_(array);

        // 在使用move语义传递后,array在这个函数调用后,它的生命周期已经完结
        // println!("array[0]: {}", array[0]) ;

        let mut array1 = vec![1, 2, 3, 4];
        fn borrow1(v: &mut Vec<i32>) {
            println!("borrow1 size of param: {}", std::mem::size_of::<&Vec<i32>>());
            v.push(10);
            for item in v {
                print!("{} ", item)
            }
            println!()
        }
        borrow1(&mut array1)
    }
}
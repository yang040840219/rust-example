#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_must_use)]
#[allow(unused_imports)]

mod thread_test {

    static N_THREADS:i32 = 10 ;
    use std::thread ;
    use std::time::Duration;

    #[test]
    fn thread_test_1() {

        let mut children = vec![] ;

        for index in 0..N_THREADS {
            let t = thread::spawn(move || {
                // index 使用了外部的环境变量
                println!("thread number :{}", index) ;
            });
            children.push(t);
        }

        for child in children {
            child.join();
        }

        println!("complete")

    }

    #[test]
    fn closure_test_1() {
        let x = 4 ;

        // 闭包可以引用外部环境变量
        let eq_1 = |z| {return z == x ;} ;

        // 函数不能引用外部环境变量
        // fn eq_2(z:i32) -> bool {
        //     return z == x ;
        // }

        let y = 4 ;
        println!("{}",eq_1(y));
    }

    #[test]
    fn closure_test_2() {
        let x = vec![1,2,3,4] ;
        let eq_1 = move |z| z == x ;

        // println!("can't use x here: {:?}", x); // variable moved due to use in closure

        let y = vec![1, 2, 3];
        println!("eq:{}", eq_1(y)) ;
    }

    #[test]
    fn closure_test_3() {
        let x = 2 ;
        let eq_1 = move |z| z == x ;
        println!("eq:{}", eq_1(x)) ;
        println!("x:{}", x);
        // 可以执行的原因是 x 是整形，实际上执行的是 copy 的操作
    }

}
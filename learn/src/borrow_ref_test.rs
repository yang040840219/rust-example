#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
mod borrow_ref_test {
    /*
      (Reference)引用
      (borrow)借用  可变引用 &mute
      & 关键字
      ref 关键字
      * 关键字
     */

    #[test]
    fn ref_1() {
        let ref a: i32;
        // a = 1 ; // expected `&i32`, found `i32`
        a = &1;
        let b = *a;
        println!("{}", a);
        println!("{}", *a);
    }

    #[test]
    fn borrow_1() {
        /*
         引用 & ， 创建一个指向值的引用， 但是并不拥有它(所有权)， 当引用离开作用域时其指向的值也不会被丢弃
         */
        fn calc_length(s: &String) -> usize {
            return s.len();
        }

        let mut s = String::from("hello");

        let s1 = &s;
        let length = calc_length(s1); // 理解为传递的是指针(引用)
        println!("length:{}", length);
        println!("s:{}", s);
        // 与＆相反的是取消引用(解引用)，是通过运算符*完成的
        println!("s1:{}", s1);
        println!("*s1:{}", *s1);

        // push_str 使 s 发生了变化，需要 s 是 可变的(mut)
        // fn append_1(s: &String) {
        //     s.push_str(" world") ;
        // }

        fn append_2(s: &mut String) {
            s.push_str(" world");
        }

        // s2 是可变的引用
        let s2 = &mut s;   // mutable borrow occurs here
        append_2(s2);

        // println!("s:{}", s);   // immutable borrow occurs here

        println!("s2:{}", s2);  // mutable borrow later used here
    }

    #[test]
    fn borrow_2() {
        let mut s = String::from("hello");

        // r1 , r2 是引用; r3, r4 是借用
        let r1 = &s ;
        let r2 = &s ;

        // 在同一时刻可以有多个不可变引用， 可变引用只能有一个
        println!("r1:{}", r1) ;
        println!("r2:{}", r2) ;

        /*
        1. Two or more pointers access the same data at the same time.
        2. At least one of the pointers is being used to write to the data.
        3. There’s no mechanism being used to synchronize access to the data.
         */
        let r3 = &mut s ;
        // fn(r3)
        let r4 = &mut s ;
        r4.push_str(" world") ;
        // 此时 r3 已经不能再提供访问，因为 r3, r4 指向了相同的数据，
        // 但是不知道r4 经过了什么操作, 再访问r3 已经不安全
        // 同理 r1 、 r2 也不能使用
        // println!("r1:{}", r1) ; // build error  immutable borrow later used here
        // println!("r2:{}", r2) ; // build error  immutable borrow later used here
        // println!("r3:{}", r3) ; // build error  first borrow later used here
        println!("r4:{}", r4) ;
    }

    #[test]
    fn ref_2() {
        /*
         build error : this function's return type contains a borrowed value,
         but there is no value for it to be borrowed from
         类似C++ 中的野指针(悬垂引用)， s:String 的作用域是 dangle(), 出了作用域会被销毁。
         但是返回了s:String 的引用
         */
        /*fn dangle() -> &String {
            let s = String::from("hello");
            return &s ;
        }

        let s = dangle();
        println!("s:{}", s);*/
    }
}
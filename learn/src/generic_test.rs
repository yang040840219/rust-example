#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
mod generic_test {
    #[derive(Debug)]
    struct User { name: String }

    #[test]
    fn array_1() {
        let array1 = [1, 2, 3, 4];
        for item in &array1 {
            println!("{}", *item);
        }

        let u1 = User { name: String::from("a") };
        let u2 = User { name: String::from("b") };
        let users = [u1, u2];
        let u = &users[0];
        // 只有实现 Deref， DerefMut 接口的数据类型才可以自动的解引用
        println!("{:?}", u);
    }

    #[test]
    fn print_array_item_type() {
        let array = [1, 2, 3, 4, 5];
        let b = &array;
        let item = b[0];
        let mut m = 1;
        for x in b.iter() {
            m = *x;
            println!("{}", &x);
        }

        for &y in b {
            m = *(&y);
        }

        let mut vec_string: Vec<String> = Vec::new();
        vec_string.push(String::from("hello"));
        let c = vec_string.get(0).unwrap();
        let mut vec_i32: Vec<i32> = Vec::new();
        vec_i32.push(1);
        let d = vec_i32.get(0).unwrap();
        println!("m:{}", m);
    }

    #[test]
    fn generic_1() {
        fn largest_i32(data: &[i32]) -> i32 {
            let mut largest = data[0];
            for &item in data {
                if largest < item {
                    largest = item;
                }
            }
            return largest;
        }

        let array = [1, 2, 3, 4, 5];
        let v = largest_i32(&array);
        println!("v:{}", v);
        println!("{}", array[4])
    }

    #[test]
    fn generic_2() {
        /*
         PartialOrd + Copy 说明泛型满足的条件
         Copy 表示具有Copy 特征 i32, f32 为了可以给 largest 赋值
         */
        fn largest<T: PartialOrd + Copy>(data: &[T]) -> T {
            let mut largest = data[0];
            for &item in data {
                if largest < item {
                    largest = item;
                }
            }
            return largest;
        }

        // let array = [1, 2, 3, 4, 5];
        // String 类型没有实现copy， 不能用泛型
        // let array = [String::from("a"), String::from("b")] ;
        let array = ["a", "b", "c"];
        let v = largest(&array);
        println!("v:{}", v);
    }

    #[test]
    fn generic_3() {
        #[derive(Debug)]
        struct Point<T, U> { x: T, y: U }

        // 泛型实例方法返回引用
        impl<T,U> Point<T, U> {
            fn get_x(&self) -> &T {
                return &(self.x);
            }

            fn get_y(&self) -> &U {
                return &(self.y);
            }
        }

        let p1 = Point { x: 4, y: 4 };
        println!("{:?}", p1);
        println!("point:{}, {}", p1.get_x(), p1.get_y())
    }
}
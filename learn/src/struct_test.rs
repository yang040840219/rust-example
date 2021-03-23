#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
mod trait_test {
    use std::fmt::Debug;

    #[test]
    fn self_test() {
        trait Shape {
            // 第一个参数是 Self 类型并且名称是 self 的称为(成员)方法， 其他的是静态函数(方法)
            // Self 代表当前实现trait 的具体类型
            // 参数对应 self: &Self
            fn area(&self) -> f64;
        }

        struct Circle {
            radius: f64
        }

        impl Shape for Circle {
            fn area(&self) -> f64 {
                // self.radius = 10.0 ;
                return std::f64::consts::PI * self.radius * self.radius;
            }
        }

        impl Circle {
            fn get_radius(&self) -> f64 {
                return self.radius;
            }

            // 类型是 Self 但是名称不是 self
            fn get_name(this: Self) -> f64 {
                println!("static method radius:{}, name:{}", this.radius, "circle");
                return this.radius;
            }
        }

        let mut c = Circle { radius: 2.0 };
        let area = c.area();
        println!("area:{}", area);
        println!("radius:{}", c.get_radius());

        // 调用静态方法
        let name = Circle::get_name(c);
        println!("name:{}", name)
    }

    #[test]
    fn implicit_test() {
        // 类似scala 中的隐式转换
        trait Double {
            fn double(&self) -> f64;
        }

        impl Double for i32 {
            fn double(&self) -> f64 {
                let x = (*self as f64) * 2.1;
                return x;
            }
        }
        let x = 2.double();
        println!("{}", x);
    }

    #[test]
    fn constraint_test() {
        fn my_print<T: Debug>(x: T) {
            println!("The value is {:?}.", x);
        }

        my_print("China");
        my_print(41_i32);
        my_print(true);
        my_print(['a', 'b', 'c'])
    }

    #[test]
    fn derive_test() {
        #[derive(Copy, Clone, Hash, Debug)]
        struct Foo { data: i32 }

        let v1 = Foo { data: 1 };
        let v2 = v1; // 相当于实现 Clone 方法
        println!("{:?}", v1);
    }

}
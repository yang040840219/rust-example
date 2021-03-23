#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
mod trait_test {
    trait GetInformation {
        fn get_name(&self) -> &String;
        fn get_age(&self) -> u32;
    }

    struct Student {
        name: String,
        age: u32,
    }

    impl GetInformation for Student {
        fn get_name(&self) -> &String {
            return &self.name;
        }

        fn get_age(&self) -> u32 {
            return self.age;
        }
    }

    #[test]
    fn student_test_1() {
        let s1 = Student { name: String::from("abc"), age: 10 };
        println!("name:{}", s1.get_name())
    }

    #[test]
    fn information_test_1() {
        // trait 作为参数
        fn print_information1(item: impl GetInformation) {
            println!("name:{}", item.get_name())
        }

        // trait bound
        fn print_information2<T:GetInformation>(item: T) {
            println!("name:{}", item.get_name())
        }

        let s1 = Student { name: String::from("abc"), age: 10 };
        print_information2(s1);
        // print_information2(s1);
    }
}
#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
mod error_test {
    /*
      panic 用于测试， 以及处理不可恢复的错误
     */

    use std::num::ParseIntError;

    #[test]
    fn give_princess() {
        fn give(gift: &str) {
            if gift == "snake" {
                // panic!("snake");
            }
            println!("receive : {}", gift);
        }
        give("snake");
    }

    #[test]
    fn give_commoner() {
        fn give(gift: Option<&str>) {
            match gift {
                Some("snake") => println!("snake"),
                Some(inner) => println!("receive:{}", inner),
                None => println!("no gift")
            }
        }

        give(Some("chicken"));
        give(None);
    }

    #[test]
    fn and_then_test() {
        #[derive(Debug)]
        enum Food {
            CordonBleu,
            Steak,
            Sushi,
        }

        fn have_ingredients(food: Food) -> Option<Food> {
            match food {
                Food::Sushi => None,
                _ => Some(food)
            }
        }

        fn have_recipe(food: Food) -> Option<Food> {
            match food {
                Food::CordonBleu => None,
                _ => Some(food),
            }
        }

        fn cookable_v2(food: Food) -> Option<Food> {
            have_ingredients(food).and_then(have_recipe)
        }

        let steak = Food::Steak;
        let food = cookable_v2(steak);
        println!("{:?}", food);
    }

    #[test]
    fn result_test() {
        /*
         Ok<T>
         Error<E>
         */

        fn multiply(n1: &str, n2: &str) -> i32 {
            // parse<F: FromStr> Result<F, F::Err>
            let f1: i32 = n1.parse::<i32>().unwrap();
            let f2: i32 = n2.parse::<i32>().unwrap();
            return f1 * f2;
        }

        let t = multiply("4", "5");
        println!("t:{}", t);
    }

    fn print_result(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn print_option(option: Option<Result<i32, ParseIntError>>) {
        match option {
            Some(result) => {
                match result  {
                    Ok(n) => println!("n:{}", n),
                    Err(e) => println!("Error: {}", e)
                }
            }
            None => println!("None")
        }
    }

    #[test]
    fn result_and_then_error() {
        // 就像 `Option` 那样，我们可以使用 `map()` 之类的组合算子。
        // 除去写法外，这个函数与上面那个完全一致，它的作用是：
        // 如果值是合法的，计算其乘积，否则返回错误。
        fn multiply(first_number_str: &str,
                    second_number_str: &str) -> Result<i32, ParseIntError> {
            first_number_str.parse::<i32>().and_then(|first_number| {
                second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
            })
        }

        let t = multiply("t", "2");
        print_result(t);
    }

    #[test]
    fn result_error_first() {
        /*
         提前返回错误
         */
        fn multiply(first_number_str: &str,
                    second_number_str: &str) -> Result<i32, ParseIntError> {
            let first_number = match first_number_str.parse::<i32>() {
                Ok(first_number) => first_number,
                Err(e) => return Err(e)
            };

            let second_number = second_number_str.parse::<i32>()?;

            return Ok(first_number * second_number);
        }

        let t = multiply("3", "2");
        print_result(t);
    }

    #[test]
    fn double_first() {
        fn df(vec: Vec<&str>) -> i32 {
            let first = vec.first().unwrap();
            return first.parse::<i32>().unwrap() * 2;
        }

        fn df1(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
            vec.first().map(|first| {
                first.parse::<i32>().map(|n| { n * 2 })
            })
        }

        let strings = vec!["10", "93", "18"];
        print_option(df1(strings));
    }

    // https://rustwiki.org/zh-CN/rust-by-example/error/multiple_error_types/define_error_type.html
}
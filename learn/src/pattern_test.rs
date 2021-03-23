#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[test]
fn match_test() {
    enum Direction {
        East, West, South, North
    }

    fn print_direction(x: Direction) {
        match x {
            Direction::East => {
                println!("east") ;
            }
            Direction::West => {
                println!("West");
            }
            Direction::South => {
                println!("South");
            }
            // Direction::North => {
            //     println!("North");
            // }
            _ => {
                println!("other") ;
            }
        }
    }

    let x = Direction::West ;
    print_direction(x);
}

#[test]
fn guard_test() {
    let x = 10;
    match x {
        i if i > 5 => println!("bigger than five"),
        i if i <= 5 => println!("small or equal to five"),
        _ => unreachable!()
    }
}
#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]

use std::io;

#[test]
pub fn guess() {
    let mut guess = String::new();
    println!("Please input your guess.");
    io::stdin().read_line(&mut guess).expect("failed to read line");
    println!("guessed:{}", guess);
}

#[test]
pub fn hello() {
    println!("hello world");
}
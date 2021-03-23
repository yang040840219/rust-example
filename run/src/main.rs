// mod 作用是导入含有 mod.rs 或者同级的 rs 文件

// 和main在同级目录下
mod math;
// 只是减少了使用方法的前缀
use math::add;

// 需要包含 mod.rs
mod math1;
use math1::math_util;
use math1::m ;


/*
  一个模块(使用mod声明) 总是可以访问父级目录，变量，函数
  create::foo  /foo 含有main.rs 、lib.rs 的目录
  super::foo  ../foo
  self::foo   ./foo
 */

fn main() {
    // 不需要使用 use / mod
    let random: u8 = rand::random();
    println!("{}", random);

    let a: i32 = add(1, 3);
    println!("a:{}", a);

    let b: i32 = math::sub(1, 3);
    println!("b:{}", b) ;

    let a1: i32 = math_util::add1(1, 3);
    println!("a1:{}", a1);

    let a2: i32 = math1::add::add2(1, 3);
    println!("a2:{}", a2);

    let b4: i32 = m::sub4(1, 3);
    println!("b4:{}", b4);

    let a4: i32 = m::add4(1, 3);
    println!("a4:{}", a4);
}



use super::sub;

pub fn sub4(x:i32, y:i32) -> i32 {
    return sub::sub2(x, y);
}

pub fn add4(x:i32, y:i32) -> i32 {
    return crate::math::add(x, y);
}
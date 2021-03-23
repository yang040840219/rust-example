#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]


#[test]
fn array_test() {
    // 数组切片
    fn mut_array(a: &mut [i32]) {
        a[2] = 5;
    }

    println!("size of &[i32; 3] : {:?}", std::mem::size_of::<&[i32; 3]>());
    println!("size of &[i32] : {:?}", std::mem::size_of::<&[i32]>());

    let mut v: [i32; 3] = [1, 2, 3];
    {
        let s: &mut [i32; 3] = &mut v;
        mut_array(s);
    }
    println!("{:?}", v);
}

#[test]
fn array_slice() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let address: &[i32; 5] = &array;

    // array 可以看做是胖指针
    fn raw_slice(array: &[i32]) {
        unsafe {
            // array 中包含了数组地址和数组长度
            let (val1, val2): (usize, usize) = std::mem::transmute(array);
            println!("Value in raw pointer:");
            println!("value1: {:X}", val1);
            println!("value2: {:X}", val2);
        }
    }
    raw_slice(address as &[i32]);
}

#[test]
fn iterator_test() {
    let v = [1, 2, 3, 4, 5];
    for (i, v) in v.iter().enumerate() {
        println!("index:{}, value:{}", i, v);
    }
    println!("{}", v[0]);
    let item = v.iter().filter(|&x| x % 2 == 0).nth(0);
    println!("{:?}", item);
}

#[test]
fn range_test() {
    let mut range = (1..10).map(|i| i * 10);
    println!("{:?}", range.nth(1));
}

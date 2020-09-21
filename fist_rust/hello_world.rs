fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 等于 500
    // tup.1 等于 6.4
    // tup.2 等于 1
    let (x, y, z) = tup;
    let a = [1, 2, 3, 4, 5];
    // a 是一个长度为 5 的整型数组

    let b = ["January", "February", "March"];
    // b 是一个长度为 3 的字符串数组

    let c: [i32; 5] = [1, 2, 3, 4, 5];
    // c 是一个长度为 5 的 i32 数组

    let d = [3; 5];
    // 等同于 let d = [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];
    // 数组访问

    //a[0] = 123; // 错误：数组 a 不可变
    let mut a = [1, 2, 3];
    a[0] = 4; // 正确
    println!("hello, world");

    // let s1 = String::from("hello");
    // let s2 = s1; 
    // println!("{}, world!", s1); // 错误！s1 已经失效

    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 is {}, s2 is {}", s1, s2);

    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    s2 = "hello xx".to_string(); // 不能直接s2 = "hello xx"  不是一个类型
    println!("s1 = {}, s2 = {}", s1, s2);
}
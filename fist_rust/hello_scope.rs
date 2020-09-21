fn main() {
    let s = String::from("hello");
    // s 被声明有效

    takes_ownership(s);
    // s 的值被当作参数传入函数
    // 所以可以当作 s 已经被移动，从这里开始已经无效

    let x = 5;
    // x 被声明有效

    makes_copy(x);
    // x 的值被当作参数传入函数
    // 但 x 是基本类型，依然有效
    // 在这里依然可以使用 x 却不能使用 s

    let s1 = gives_ownership();
    // gives_ownership 移动它的返回值到 s1

    let s2 = String::from("hello");
    // s2 被声明有效

    let s3 = takes_and_gives_back(s2);
    // s2 被当作参数移动, s3 获得返回值所有权


    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    /*
    引用不会获得值的所有权。

    引用只能租借（Borrow）值的所有权。

    引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权：
    */
    println!("The length of '{}' is {}.", s1, len);


    let mut s1 = String::from("run");
    // s1 是可变的

    // let s2 = &s1;
    // s2.push_str("oob"); // 错误，禁止修改租借的值

    let s2 = &mut s1;
    // s2 是可变的引用
    s2.push_str("oob");
    
    println!("{}", s2);

} 
// 函数结束, x 无效, 然后是 s. 但 s 已被移动, 所以不用被释放
// s3 无效被释放, s2 被移动, s1 无效被释放.


fn takes_ownership(some_string: String) {
    // 一个 String 参数 some_string 传入，有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string 被声明有效

    return some_string;
    // some_string 被当作返回值移动出函数
}

fn takes_and_gives_back(a_string: String) -> String { 
    // a_string 被声明有效

    a_string  // a_string 被当作返回值移出函数
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
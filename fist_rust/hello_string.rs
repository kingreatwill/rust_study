fn main(){

    // 新建字符串：
    let string = String::new();

    // 基础类型转换成字符串：
    let one = 1.to_string();         // 整数到字符串
    let float = 1.3.to_string();     // 浮点数到字符串
    let slice = "slice".to_string(); // 字符串切片到字符串

    // 包含 UTF-8 字符的字符串：
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // 字符串追加：
    let mut s = String::from("run");
    s.push_str("oob"); // 追加字符串切片
    s.push('!');       // 追加字符

    // 用 + 号拼接字符串：
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    // 这个语法也可以包含字符串切片：
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // 使用 format! 宏：
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // 字符串长度：
    let s = "hello";
    let len = s.len();

    // 这里 len 的值是 5。
    let s = "你好";
    let len = s.len();

    // 这里 len 的值是 6。因为中文是 UTF-8 编码的，每个字符长 3 字节，所以长度为6。
    // 但是 Rust 中支持 UTF-8 字符对象，所以如果想统计字符数量可以先取字符串为字符集合：
    let s = "hello你好";
    let len = s.chars().count();
}
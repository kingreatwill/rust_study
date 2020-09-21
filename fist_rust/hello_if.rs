fn main() {
    let number = 3;
    if number < 5 {
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }

    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    }  
    else if a < 0 {
        b = -1;
    }  
    else {
        b = 0;
    }
    println!("b is {}", b);

    let a = 3;
    let number = if a > 0 { 1 } else { -1 };
    println!("number 为 {}", number);

    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");

    let mut i = 0;
    while i < 10 {
        // 循环体
        i += 1;
    }
    println!("{}", i);

    let a = [10, 20, 30, 40, 50];
    for i in 0..a.len() {
        println!("a[{}] = {}", i, a[i]);
    }
    
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为 : {}", i);
    }



    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }
    // loop 循环可以通过 break 关键字类似于 return 一样使整个循环退出并给予外部一个返回值。

    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch == 'O' {
            break i;
        }
        i += 1;
    };
    println!(" \'O\' 的索引为 {}", location);
}
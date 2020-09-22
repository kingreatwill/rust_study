use std::io::stdin;
use std::fs;

fn main() {
    let args = std::env::args();
    println!("{:?}", args);
    for arg in args {
        println!("{}", arg);
    }

    let mut str_buf = String::new();
    stdin().read_line(&mut str_buf)
        .expect("Failed to read line.");

    println!("Your input line is \n{}", str_buf);

    fs::write("D:\\text.txt", "FROM RUST PROGRAM")
        .unwrap();// 因为它会直接删除文件内容（无论文件多么大）。如果文件不存在就会创建文件。

    let text = fs::read_to_string("D:\\text.txt").unwrap();
    // let content = fs::read("D:\\text.txt").unwrap();//我们可以用 std::fs::read 函数读取 u8 类型集合：
    println!("{}", text);
}

/*
Rust 中的文件流读取方式：

实例
use std::io::prelude::*;
use std::fs;

fn main() {
    let mut buffer = [0u8; 5];
    let mut file = fs::File::open("D:\\text.txt").unwrap();
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
}

如果想使用流的方式写入文件内容，可以使用 std::fs::File 的 create 方法：

实例
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::create("D:\\text.txt").unwrap();
    file.write(b"FROM RUST PROGRAM").unwrap();
}
注意：打开的文件一定存放在可变的变量中才能使用 File 的方法！

File 类中不存在 append 静态方法，但是我们可以使用 OpenOptions 来实现用特定方法打开文件：

实例
use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
   
    let mut file = OpenOptions::new()
            .append(true).open("D:\\text.txt")?;

    file.write(b" APPEND WORD")?;

    Ok(())
}

OpenOptions 是一个灵活的打开文件的方法，它可以设置打开权限，除append 权限以外还有 read 权限和 write 权限，如果我们想以读写权限打开一个文件可以这样写：

实例

use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
   
    let mut file = OpenOptions::new()
            .read(true).write(true).open("D:\\text.txt")?;

    file.write(b"COVER")?;

    Ok(())
}
*/
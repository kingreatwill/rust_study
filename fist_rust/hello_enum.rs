#[derive(Debug)]
enum Book1 {
    Papery, Electronic
}

#[derive(Debug)]
enum Book2 {
    Papery(u32),
    Electronic(String),
}

#[derive(Debug)]
enum Book3 {
    Papery { index: u32 },
    Electronic { url: String },
}



fn main() {
    let book = Book1::Papery;
    println!("{:?}", book);
    
    let book = Book2::Papery(1001);
    let ebook = Book2::Electronic(String::from("url://..."));
    println!("{:?}", book);
    println!("{:?}", ebook);
    
    let book = Book3::Papery{index: 1001};
    println!("{:?}", book);


    match book {
        Book3::Papery { index } => {
            println!("Papery book {}", index);
        },
        Book3::Electronic { url } => {
            println!("E-book {}", url);
        }
    }
  
    let t = "abc";
    match t {
        "abc" => println!("Yes"),
        _ => {},
    }
    let i = 0;
    match i {
        0 => println!("zero"),
        _ => {},
    }
    let i = 0;
    if let 0 = i {
        println!("zero");
    }

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
    // 如果你的变量刚开始是空值，你体谅一下编译器，它怎么知道值不为空的时候变量是什么类型的呢？
    // 所以初始值为空的 Option 必须明确类型：
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }

    let t = Some(64);
    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }

    let book = Book2::Electronic(String::from("url"));
    if let Book2::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }
}
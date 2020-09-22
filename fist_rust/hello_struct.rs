#[derive(Debug)]
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 结构体方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 一直使用的 String::from 函数就是一个"关联函数"。
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main(){

    let runoob1 = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013,
    };
    
    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    let runoob = Site {
        domain,  // 等同于 domain : domain,
        name,    // 等同于 name : name,
        nation: String::from("China"),
        found: 2013,
    };
    
    let site = Site {
        domain: String::from("www.runoob.com1"),
        name: String::from("RUNOOB"),
        ..runoob //..runoob 后面不可以有逗号。这种语法不允许一成不变的复制另一个结构体实例，意思就是说至少重新设定一个字段的值才能引用其他实例的值。
    };

    println!("{} {}",site.domain,site.found);

    //元组结构体
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);

    let rect1 = Rectangle { width: 30, height: 50 };
    // 如第一行所示：一定要导入调试库 #[derive(Debug)] ，
    // 之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体：
    // 如果属性较多的话可以使用另一个占位符 {:#?} 。
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    // 结构体方法
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1's area is {}", rect1.area());

    // 一直使用的 String::from 函数就是一个"关联函数"。
    let rect = Rectangle::create(30, 50);
    println!("{:?}", rect);
    /*
    贴士：结构体 impl 块可以写几次，效果相当于它们内容的拼接！

    单元结构体
    结构体可以值作为一种象征而无需任何成员：

    struct UnitStruct;
    我们称这种没有身体的结构体为单元结构体（Unit Struct）。
    */
}
/*
特性
特性（trait）概念接近于 Java 中的接口（Interface），但两者不完全相同。
特性与接口相同的地方在于它们都是一种行为规范，可以用于标识哪些类有哪些方法。
默认特性
这是特性与接口的不同点：接口只能规范方法而不能定义方法，但特性可以定义方法作为默认方法，
因为是"默认"，所以对象既可以重新定义方法，也可以不重新定义方法使用默认的方法：
*/

trait Descriptive {
    fn describe(&self) -> String;
    fn describe2(&self) -> String {
        String::from("[Object]")
    }
}

struct Person {
    name: String,
    age: u8
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}
/*
格式是：

impl <特性名> for <所实现的类型名>
Rust 同一个类可以实现多个特性，每个 impl 块只能实现一个。
*/


trait Descriptive2 {
    fn describe(&self) -> String {
        String::from("[Object]")
    }
}

struct Person2 {
    name: String,
    age: u8
}
impl Descriptive2 for Person2{}


trait Comparable {
    fn compare(&self, object: &Self) -> i8;
}

fn max<T: Comparable>(array: &[T]) -> &T {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i].compare(&array[max_index]) > 0 {
            max_index = i;
        }
        i += 1;
    }
    &array[max_index]
}

impl Comparable for f64 {
    fn compare(&self, object: &f64) -> i8 {
        if &self > &object { 1 }
        else if &self == &object { 0 }
        else { -1 }
    }
}

/*
struct A<T> {}

impl<T: B + C> A<T> {
    fn d(&self) {}
}
这段代码声明了 A<T> 类型必须在 T 已经实现 B 和 C 特性的前提下才能有效实现此 impl 块。

*/


fn main() {
    let cali = Person {
        name: String::from("Cali"),
        age: 24
    };
    println!("{}", cali.describe());
    println!("{}", cali.describe2());
    //output1(cali);
    output2(cali);
   



    let cali2 = Person2 {
        name: String::from("Cali2"),
        age: 24
    };
    println!("{}", cali2.describe());

    let arr = [1.0, 3.0, 5.0, 4.0, 2.0];
    println!("maximum of arr is {}", max(&arr));
}
// 特性做参数
fn output1(object: impl Descriptive) {
    println!("{}", object.describe());
}

fn output2<T: Descriptive>(object: T) {
    println!("{}", object.describe());
}

fn output_two<T: Descriptive>(arg1: T, arg2: T) {
    println!("{}", arg1.describe());
    println!("{}", arg2.describe());
}
/*
特性作类型表示时如果涉及多个特性，可以用 + 符号表示，例如：

fn notify(item: impl Summary + Display)
fn notify<T: Summary + Display>(item: T)

复杂的实现关系可以使用 where 关键字简化，例如：

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U)
可以简化成：

fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug


特性做返回值
特性做返回值格式如下：

实例
fn person() -> impl Descriptive {
    Person {
        name: String::from("Cali"),
        age: 24
    }
}
但是有一点，特性做返回值只接受实现了该特性的对象做返回值且在同一个函数中所有可能的返回值类型必须完全一样。比如结构体 A 与结构体 B 都实现了特性 Trait，下面这个函数就是错误的：

实例
fn some_function(bool bl) -> impl Descriptive {
    if bl {
        return A {};
    } else {
        return B {};
    }
}
*/
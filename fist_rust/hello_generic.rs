fn max1(array: &[i32]) -> i32 {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}

/*
Rust 中的结构体和枚举类都可以实现泛型机制。

struct Point<T> {
    x: T,
    y: T
}
这是一个点坐标结构体，T 表示描述点坐标的数字类型。我们可以这样使用：

let p1 = Point {x: 1, y: 2};
let p2 = Point {x: 1.0, y: 2.0};
使用时并没有声明类型，这里使用的是自动类型机制，但不允许出现类型不匹配的情况如下：

let p = Point {x: 1, y: 2.0};
x 与 1 绑定时就已经将 T 设定为 i32，所以不允许再出现 f64 的类型。如果我们想让 x 与 y 用不同的数据类型表示，可以使用两个泛型标识符：

struct Point<T1, T2> {
    x: T1,
    y: T2
}
*/

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn main() {
    let a = [2, 4, 6, 3, 1];
    println!("max = {}", max1(&a));

    let p = Point { x: 1, y: 2 };
    println!("p.x = {}", p.x());
    
    let p = Point { x: 1.1, y: 2.1 };
    println!("p.y = {}", p.y());
    println!("p.x = {}", p.x());
}
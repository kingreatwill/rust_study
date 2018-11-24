fn main() {
    let mut a: f64 = 1.0;
    let b = 2.0f32;

    //改变 a 的绑定
    a = 2.0;
    println!("{:?}", a);

    //重新绑定为不可变
    let a = a;

    //不能赋值
    //a = 3.0;

    //类型不匹配
    //assert_eq!(a, b);




    let (c, mut d): (bool,bool) = (true, false);
    println!("c = {:?}, d = {:?}", c, d);
    //c 不可变绑定
    //c = false;

    //d 可变绑定
    d = true;
    assert_eq!(c, d);
}
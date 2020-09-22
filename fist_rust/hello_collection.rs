use std::collections::HashMap;
fn main() {
    let mut vector = vec![1, 2, 4, 8];
    vector.push(16);
    vector.push(32);
    vector.push(64);
    println!("{:?}", vector);

    let mut v1: Vec<i32> = vec![1, 2, 4, 8];
    let mut v2: Vec<i32> = vec![16, 32, 64];
    v1.append(&mut v2);
    println!("{:?}", v1);

    let mut v = vec![1, 2, 4, 8];
    println!("{}", v[1]);
    // 因为向量的长度无法从逻辑上推断，get 方法无法保证一定取到值，所以 get 方法的返回值是 Option 枚举类，有可能为空。
    println!("{}", match v.get(0) {
        Some(value) => value.to_string(),
        None => "None".to_string()
    });

    let v = vec![100, 32, 57];
    for i in &v {
            println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }



    let mut map = HashMap::new();
    map.insert("color", "red");
    map.insert("size", "10 m^2");
    println!("{}", map.get("color").unwrap());

    for p in map.iter() {
        println!("{:?}", p);
    }
    /*
    Rust 的映射表是十分方便的数据结构，当使用 insert 方法添加新的键值对的时候，如果已经存在相同的键，
    会直接覆盖对应的值。如果你想"安全地插入"，就是在确认当前不存在某个键时才执行的插入动作，可以这样：
    map.entry("color").or_insert("red");
    */

    let mut map = HashMap::new();
    map.insert(1, "a");
   
    if let Some(x) = map.get_mut(&1) {
        *x = "b";
    }
    println!("{:?}", map);

}
// 数组实现#[derive(Debug)]
// https://doc.rust-lang.org/std/primitive.array.html
pub fn array_demo() {
    let mut array: [i32; 3] = [0; 3];
    let mut array2: [i32; 3] = [0; 3];
    println!("{}", array == array2);
    array[1] = 1;
    array[2] = 2;

    assert_eq!([1, 2], &array[1..]);

    // error: for x in array { }
    // ok: for x in array.iter() { }
    // ok: for x in &array { }
    // This loop prints: 0 1 2
    for x in &array {
        print!("{} ", x);
    }
}




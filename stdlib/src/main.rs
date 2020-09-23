#[path = "./primitive_types/array.rs"] mod array;
mod rust2018;

fn main() {
    println!("Hello, world!");
    rust2018::demo::demo();
    array::array_demo();
}

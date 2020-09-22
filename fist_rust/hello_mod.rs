mod nation {
    pub mod government {
        pub fn govern() {println!("{}","govern")}
    }

    mod congress {
        pub fn legislate() {}
    }
   
    mod court {
        fn judicial() {
            super::congress::legislate();
        }
    }
}


mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}


mod SomeModule {
    pub enum Person {
        King {
            name: String
        },
        Quene
    }
}

mod second_module;

mod nation2 {
    pub mod government2 {
        pub fn govern2() {println!("use 关键字1")}
    }
    
    pub fn govern2() {println!("use 关键字2")}

    pub use self::government2::govern2 as nation_govern;
}
// use 关键字
use crate::nation2::government2::govern2;
use crate::nation2::govern2 as nation_govern;
use std::f64::consts::PI; //引用标准库
fn main() {
    nation::government::govern();
    eat_at_restaurant();

    let person = SomeModule::Person::King{
        name: String::from("Blue")
    };
    match person {
        SomeModule::Person::King {name} => {
            println!("{}", name);
        }
        _ => {}
    };
    println!("{}", second_module::message());

    govern2();
    nation2::nation_govern();
    nation_govern();

    println!("{}", (PI / 2.0).sin());
}
use back_of_house::Breakfast;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        mod back_of_house_sub {
            fn fix_incorrect_order() {
                cook_order();
                super::serve_order();       // superを使って相対パスのルートを指定 -> そこからserve_orderを見つけ出す
            }
        
            fn cook_order() {}
        }
    }
}


mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        sessional_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                sessional_fruit: String::from("peaches"),
            }
        }
    }
}


// enumはそれぞれにpubをつけなくても公開される
mod back_of_house3 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


// front_of_houseはeat_at_restaurantと兄弟なのでpubをつけなくても使える!
pub fn eat_at_restaurant() {
    // Absolute Path(絶対パス)
    crate::front_of_house::hosting::add_to_waitlist();

    // Related Path(相対パス)
    front_of_house::hosting::add_to_waitlist();

    // #######################################

    // 夏 (summer) にライ麦 (Rye) パン付き朝食を注文
    let mut meal: Breakfast = back_of_house::Breakfast::summer("Rye");

    // やっぱり別のパンにする
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // フルーツを変えようとするとコンパイルエラーになる(公開されてないから．pubをつけてない)
    // meal.seasonal_fruit = String::from("blueberries");

    // #######################################
    let order1 = back_of_house3::Appetizer::Soup;
    let order2 = back_of_house3::Appetizer::Salad;
}


fn main() {
    eat_at_restaurant();
}
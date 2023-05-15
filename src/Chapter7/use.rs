mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// useでモジュールをスコープに持ち込む
use crate::front_of_house::hosting;
// 上のuse文と等しい
// use self::front_of_house::hosting;
// Rustの慣例的に，以下のようにはしない
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // スコープに持ち込んでいるので，hostingからでok
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// ##################################

// asを使って新しい名前を与える
use std::fmt::Result;
use std::io::Result as IoResult;
// fn function1() -> Result {
// }
// fn function2() -> IoResult {
// }

// ##################################

// useのリストをネストして整理する
use std::cmp::Ordering;
use std::io;
// 上記の2行をネストして1行で表す
// use std::{cmp::Ordering, io};

fn main() {

}
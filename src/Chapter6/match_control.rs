// 25セント硬貨は州によって異なる効果を作っていたらしいからそれ用の列挙型
#[derive(Debug)] // デバッグ用
enum UsState {
    Alabama,
    Alaska,
    // ...
}


// enumにて列挙型を定義
enum Coin {
    Penny,      // 1セント硬貨
    Nickel,     // 5セント硬貨
    Dime,       // 10セント硬貨
    Quarter(UsState),    // 25セント硬貨
}

// 1
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 波括弧で囲うことで複数行の処理が可能
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}


// 2 Optionを用いたマッチ
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),       // iは5になってSome(6)を生成し返す
    }
}


fn main() {
    // 1 matchを用いた例
    let coin = Coin::Quarter(UsState::Alabama);
    value_in_cents(coin);

    // ################################

    // 2 Optionを用いたマッチの使用例
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // ################################
    
    // ただ一つの値とマッチする時のみ処理を走らせるmatch式
    let some_u8_value: Option<u8> = Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // 上記のmatch式をif let式に置き換えて短く記載
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
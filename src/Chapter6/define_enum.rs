// 1 enumを使用して列挙型を定義
enum IpAddKind {
    V4,
    V6,
}

// 列挙型を引数とした関数の定義も可能
fn route(iptype: IpAddKind) { }

// 2 新しい列挙型を定義
enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,                           // 紐づけられたデータはなし
    Move { x: i32, y: i32 },        // 中に匿名構造体を含む
    Write(String),                  // 単独のStringオブジェクトを含む
    ChangeColer(i32, i32, i32),     // 3つのi32を含む
}

// 3 enumを使ってメソッドを定義する
impl Message {
    fn call(&self) {
        // ここでメソッドを定義する
    }
}

fn main() {
    // 1 宣言した列挙型からインスタンスを生成
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    // 2 新しく定義した列挙型からインスタンスを生成
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // 3 enumを使ったメソッド例
    let m = Message::Write(String::from("hello"));
    m.call();   // 定義したcallメソッドを使用

    // Optionを使ってみる
    let some_num = Some(5);
    let some_string = Some("aaa");
    let none: Option<i32> = None;
}
// 長方形の高さと幅の情報を持つ構造体を定義
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    // 単純に長方形の高さと幅変数を宣言する
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} sqyare pixels.", area_var(width1, height1));

    // 上の表現をタプルを使って表す
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} sqyare pixels.", area_tuple(rect1));

    // 上の表現を構造体を使って表す
    let rect2 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} sqyare pixels.", area_struct(&rect2));
    println!("rect2 is {:#?}", rect2);      // rect2のデバッグ
}


// 変数を使って面積を計算する関数
fn area_var(width: u32, height: u32) -> u32 {
    width * height
}


// タプルを使って面積を計算する関数
fn area_tuple(dimentions: (u32, u32)) -> u32 {
    // 添え字で高さと幅にアクセスするからわかりずらい!
    dimentions.0 * dimentions.1
}


// 構造体を使って面積を計算する関数
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
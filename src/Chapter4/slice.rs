// 文字列リテラルはスライスであるため，引数も&strにしてやる
// イミュータブルな借用なのでsは変更不可
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();       // Stringオブジェクトをバイト配列に変換する

    // バイト配列をenumerateによってラップする(i: 最初からの添字，&item: 添字に対応する1バイト)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {       // 文字が空白であれば
            return &s[..i];     // 最初から空白までをスライスして返す
        }
    }
    &s[..]                      // 空白がなければ文字を全て返す
}
 
fn main() {
    let s = String::from("hello, world!");
    let word = first_word(&s[..]);
    println!("The first word of s is '{}'", word);

    // ###################################

    // 配列のスライスの例
    let a = [1, 2, 3, 4, 5];
    for (i, &item) in a.iter().enumerate() {
        print!("a[{}] is {}. ", i, &item);
    }
    println!();
    let a_slice = &a[1..3];
    for (i, &item) in a_slice.iter().enumerate() {
        print!("a_slice[{}] is {}. ", i, &item);
    }
    println!()
}
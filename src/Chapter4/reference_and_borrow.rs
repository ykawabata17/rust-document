fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);        // 引数に&がついたことに注意! これが参照渡し
    // 所有権の受け渡しがないためエラーにならない

    println!("The length of '{}' is '{}'", s1, len);

    // ###########################################

    // 可変な参照と不変な参照
    let mut s2 = String::from("hello");
    change_ok(&mut s2);
    println!("s2 is '{}'", s2);
}

// このように関数の引数に参照を取ることを借用と呼ぶ
fn calculate_length(s: &String) -> usize {
    s.len()
} // 通常であればsはスコープ外になり所有権がなくなるが，参照しているだけなので所有権を持っておらず，何も起こらない

// 借用したものを変更しようとしたらエラーになる
// fn change_ng(s: &String) {
//     s.push_str(", world");
// }

// 可変な参照であるため変更可能
fn change_ok(s: &mut String) {
    s.push_str(", world");
}
fn main() {
    let s1 = "hello";        // 不変で可変な変数にできない(mutを使ってもダメ)
    println!("s1 is {}", s1);

    let s2 = String::from("hello");     // String型を使用した例
    println!("s2 is {}", s2);

    let mut s3 = String::from("hello"); // 可変変数にできる
    s3.push_str(", world!");            // 可変なので，文字列に追加
    println!("s3 is {}", s3);

    // ############################

    // 可変なString型変数を別の変数にコピーする例
    let mut s4 = String::from("hello");
    let s4_copy = s4;                       // s4をs4_copyに移す(ここからs4は使用できない)
    println!("s4_copy is {}", s4_copy);     // s4の所有権はs4_copyに移ったので使用可能
    // println!("s4 is {}", s4);            // s4の所有権が自分になくなったので使うことができない

    let s4_clone = s4_copy.clone();         // String型のdeep copy
    println!("s4_copy is {}, s4_clone is {}", s4_copy, s4_clone);

    // ############################

    // 関数を使用した例
    let mut s5 = String::from("function sample");
    takes_owner(s5);    // 関数に値を渡したので，s5の所有権が渡される．(s5はこれ以降使えない)

    // 所有権を渡して返ってくる例
    let mut s6 = String::from("take and give");
    let s_give = takes_and_gives_owner(s6);     // ここでs6の所有権がムーブするが，s_stringからs_giveにムーブされる
}


fn takes_owner(s_copy: String) {
    println!("s is {}", s_copy);
}   // s_copyの所有権がなくなる

fn takes_and_gives_owner(s_string: String) -> String {
    s_string // s_stringの所有権がムーブする
}
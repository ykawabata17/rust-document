fn main() {
    let mut s1 = String::new();

    let content = "initial comment";
    let s2 = content.to_string();
    // 1行で書くと以下の2パターンのようになる
    // let s = "initial comment".to_string();
    // let s = String::from("initial comment");

    // ##################################

    // 文字列を更新する
    let mut s3 = String::from("foo");
    s3.push_str("bar");
    println!("s3 is {}", s3);

    let mut s4 = String::from("lo");
    s4.push('l');
    println!("s4 is {}", s4);

    // ##################################

    // +演算子 (s_firstは所有権を失い，s_secondはそのまま)
    let s_first = String::from("Hello, ");
    let s_second = String::from("world!");
    let s_add = s_first + &s_second;
    println!("s_add is {}", s_add);

    // format!マクロ
    let s_one = String::from("tic");
    let s_two = String::from("tac");
    let s_three = String::from("toe");
    // let s_all = s_one + "-" + &s_two + "-" + &s_three;
    // +演算子では先頭の所有権を奪うが，format!マクロでは奪わずに実装可能
    let s_all = format!("{}-{}-{}", s_one, s_three, s_three);
    println!("s_all is {}", s_all);

    // ##################################

    // 文字列に添字アクセス
    let s5 = String::from("hello");
    let second = &s5[1..2];
    println!("second is {}", second);
    // for を使って1文字ずつピックアップ
    for v in s5.chars() {
        print!("{} ", v);
    }
    println!();
}
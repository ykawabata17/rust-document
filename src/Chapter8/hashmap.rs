pub use std::collections::HashMap;
use std::hash::Hash;


fn main() {
    let mut scores1 = self::HashMap::new();
    scores1.insert(String::from("Blue"), 10);       // キーがBlueで値が10のセットを追加
    scores1.insert(String::from("Yellow"), 50);

    // ベクタからハッシュマップを作成
    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10, 50];
    // zipメソッドをツァってタプルのベクタを作り，collectメソッドでハッシュマップに変換する
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // これ以降はfield_nameとfield_valueは所有権がないため使用できない
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }

    // キーに値がなかった場合に値を挿入する
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 20);
    scores2.entry(String::from("Blue")).or_insert(50);
    scores2.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores2);
}
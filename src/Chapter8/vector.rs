fn main() {
    // ベクタを生成する(型注釈をつけないとコンパイルエラー)
    let v1: Vec<i32> = Vec::new();

    // vec!マクロを用いて生成する(値がすでに入っていて型を推測できるから型注釈不要)
    let v2 = vec![1, 2, 3];

    // ミュータブルなベクタを生成(のちに値を入れるため型注釈不要)
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);

    // ###############################

    // ベクタの要素を読む
    let v4 = vec![1, 2, 3, 4, 5];
    // &と[]を使用して参照を得る方法
    // この場合は存在しない場所を参照しようとすればエラーになる
    let third: &i32 = &v4[2];
    println!("The third element is {}", third);
    // getメソッドに引数として添字を渡してOption<&T>を得る方法
    // こっちの方法は存在しない場所を参照しようとしてもエラーにならない
    match v4.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // ###############################

    // ベクタ内の値を順に処理する
    let v = vec![10, 20, 30];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;       // *は参照外し演算子
        println!("{}", i);
    }

    // ###############################

    // vector内で複数の方を使いたい
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("Hello")),
    ];
}
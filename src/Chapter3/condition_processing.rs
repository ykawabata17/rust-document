fn main() {
    // if文を1行で書く
    let condition = true;
    let num = if condition {5} else {6};
    println!("The value of number is {}", num);

    // ############################

    // for文で連番を出力する
    for num in 1..10 {
        println!("{}", num);
    }
    // for文で連番を逆順に出力する
    for num in (1..10).rev() {
        println!("{}", num);
    }

    // ############################

    // (練習)この章の練習問題の類でよくある九九の表を出力するプログラムを作ってみる
    for i in 1..10 {
        for j in 1..10 {
            print!("{} ", i*j);
        }
        println!();
    }
    // 最後に半角スペースがあるけど気にしないでおく．．．
}

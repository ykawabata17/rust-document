// 引数の値に5を足して返す関数
fn sum_five(num: i32) -> i32 {
    // 以下の文はセミコロンがないため"式"として認識される
    num + 5
    // セミコロンをつけると"文"として認識されてコンパイルが通らない
    // num + 5;
}


fn main() {
    // sum_five関数に引数を渡して5を足された値を返してもらう
    let num = sum_five(30);
    println!("The number of 30 + 5 is {}", num);
}

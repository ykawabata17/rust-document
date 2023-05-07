fn main() {
    // タプルの分配
    let tup = (100, 3.2, 1);
    let (x, y, z) = tup;
    println!("The value of x, y, z is {}, {}, {}", x, y, z);

    // ############################
    
    // タプルの要素へのアクセス
    let x: (i32, f64, u8) = (500, 6.4, 1);
    println!("first is {}, second is {}, third is {}", x.0, x.1, x.2);

    // ############################

    // 配列の様々な定義の方法
    let a: [i32; 5] = [1, 2, 3, 4, 5];          // 5つの要素を持つi32型の配列を定義
    let a = [3; 5];                             // 5つの要素を持ち，全て3で初期化して配列を定義
}

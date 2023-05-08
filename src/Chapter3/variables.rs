fn main() {
    // let mut x = 5;       // 不変変数を定義(コンパイルエラーになる)
    let mut x = 5;          // 可変変数を定義
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // ############################

    // 不変変数の覆い隠しの例
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of in the inner scope is {}", x);
    }
    println!("The value of x is {}", x);

    // ############################

    // 不変変数であるが，覆い隠しによって変数名は同じであるが型が変わる例
    let spaces = "    ";
    println!("Spaces is {}", spaces);
    let spaces = spaces.len();
    println!("Spaces is {}", spaces);

    // ############################

    
}

// 構造体Userの定義
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 特定のフィールドは固定でインスタンスを作成するようにする関数
fn build_user(username: String, email: String) -> User {
    User {
        username,   // username: username,　と同じ
        email,      // email: email,　と同じ
        sign_in_count: 1,
        active: true,
    }
}


fn main() {
    // User構造体のインスタンスを作成
    let mut user1 = User {
        username: String::from("username1"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };

    // user1はミュタブル変数であるため，フィールド値を変更する
    user1.email = String::from("anyone@example.com");

    let user2 = build_user(String::from("username2"), String::from("someone2@example.com"));

    // ############################################

    // 構造体更新記法を使う
    // let user3 = User {
    //     username: String::from("username3"),
    //     email: String::from("someone3@example.com"),
    //     active: user1.active,
    //     sign_in_count: user1.sign_in_count,
    // };
    // 構造体更新記法を使用して以下に書き換えることができる
    let user3 = User {
        username: String::from("username3"),
        email: String::from("someone3@example.com"),
        ..user1
    };

    // ############################################

    // 名前付きであるがフィールドのないタプル構造体を使う
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
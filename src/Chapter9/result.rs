use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("There was a proble, opening the file: {:?}", error)
        },
    };

    // ####################################

    // unwrap()はOkの場合そのまま，Errの場合はpanic!マクロを呼んでくれる
    let f = File::open("hello1.txt").unwrap();
    // expect()は，unwrap()に似ているが，Errの場合のコメントを指定できる
    let f = File::open("hello2.txt").expect("Failed to open hello2.txt");
}


fn read_user_name_from_file() -> Result<String, io::Error> {
    let f = File::open("hello3.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


// ?演算子を使用して，上の関数を簡単に書く
fn read_user_name_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello3.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
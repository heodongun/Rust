use std::io;

fn main() {
    let correct_password: i64 = 1; 
    loop {
        println!("비밀번호를 입력하세요:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("입력 읽기 실패");
        let password: i64 = match input.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("숫자를 입력하세요!");
                continue; 
            }
        };
        if password == correct_password {
            println!("어케 맞췄노!");
            break;
        }
        println!("틀렸노ㅋ");
    }
}

use std::io;

pub fn get_user_input() -> String {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();

    user_input
}

pub fn get_trimed_user_input() -> String {
    get_user_input().trim().to_owned()
}

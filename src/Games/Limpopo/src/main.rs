use std::io;
use std::process::exit;

fn main() {
    let mut user_ZOV = String::new();
    let mut user_ZOV_num: i64;
    loop {
        println!("Enter random number: ");
        io::stdin().read_line(&mut user_ZOV).expect("bad input");
        user_ZOV_num = user_ZOV.trim().parse().expect("bad input");

        println!("You lose. My number is bigger than yours: {}", user_ZOV_num + 1);

        user_ZOV.clear();

        println!("Want to play one more time? (Enter 1): ");
        io::stdin().read_line(&mut user_ZOV).expect("bad input");

        println!("{}", user_ZOV);
        user_ZOV_num = user_ZOV.trim().parse().expect("bad input");

        if user_ZOV_num != 1 {
            exit(0);
        }
        user_ZOV.clear();
    }
}

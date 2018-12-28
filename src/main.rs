extern crate rpassword;

use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::fs::File;

fn main() {
    print!("{}[2J", 27 as char);
    print!("Please input a dictionary path: \0");
    match io::stdout().flush() {
        Ok(o) => o,
        Err(_) => {} 
    }
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).unwrap();
    let pass = rpassword::prompt_password_stdout("Password: ").unwrap();
    let dict = File::open(file_path.trim()).unwrap();
    let mut err_count: u128 = 0;
    let mut line_num: u128 = 0;
    for line in BufReader::new(dict).lines() {
        match line {
            Ok(o) => process(o, &pass, &file_path, &line_num),
            Err(_) => err_count = err_count + 1
        }
        line_num = line_num + 1;
        if line_num % 1000000 == 0 {
            print!("{}[2J", 27 as char);
            if err_count > 0 {
                println!("Skipped {} passwords due to non-ascii character.", &err_count);
            }
            println!("Checking password #{}", &line_num);
        }
    }
    println!("Password not found in dictionary!");
}

fn process(guess: String, pass: &String, path: &String, line: &u128) {
    if guess.as_str() == pass.as_str() {
        println!("Password found `{}` @ line {} in {}", guess, line, path);
        std::process::exit(0);
    }
}

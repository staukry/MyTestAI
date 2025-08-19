
use mai::A::data::*;
use std::io::{self, stdin};

fn main() {
    loop {
        let mut buffer: String = String::new();
        let _:String= stdin().read_line(&mut buffer).expect("a").to_string();
        let input = buffer.trim().to_string();
        let Results: Result<Vec<String>, io::Error> = read_csv(input);
        println!("{:?}", Results);
    }
    
}
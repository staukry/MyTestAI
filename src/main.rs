#![allow(non_snake_case)]
use mai::A::data::*;
use std::{collections::HashMap, io::{self, stdin}};

fn main() {
    // let mut table: HashMap<u32, Vec<f64>> = HashMap::new(); <---- this code is table code
    loop {
        let mut buffer: String = String::new();
        let _:String= stdin().read_line(&mut buffer).expect("a").to_string();
        let input = buffer.trim().to_string();
        let Results: Result<HashMap<u32, Vec<String>>, io::Error> = read_csv(input);
        println!("{:#?}", Results);
    }
}
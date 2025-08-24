#![allow(non_snake_case)]


pub mod Cacls {
    pub mod cacls{
        pub fn Euclidean_distance(Target1: (f64, f64),Target2: (f64, f64)) -> f64 {
            let F = Target1.0 - Target2.0;
            let S = Target1.1 - Target2.1;
            (F*F + S*S).sqrt()
        }
    }
}

pub mod A {
    pub mod data{
        use std::fs::File;
        use std::io::Read;
        use std::collections::HashMap;
        pub fn read_csv(Input:String) -> std::io::Result<HashMap<u32, Vec<String>>> {
            let mut F = File::open(Input)?;
            let mut data = String::new();
            F.read_to_string(&mut data)?;
            let _ = data.retain(|c| c != '\r');
            let mut table: HashMap<u32, Vec<String>> = HashMap::new();
            let lines= data.lines();
            let mut line_number:u32 = 0;
            for Line in lines {
                table.insert(line_number, Line.split(',').map(|T| T.to_string()).collect());//vec!( Vec![],Vec![]......) <------- ????
                // println!("{:?}", table);
                line_number += 1;
            }
            Ok(table)
        }
    }
    pub mod K_Series {
        pub struct KNN {
            //uhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
        }
    }
}
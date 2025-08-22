#![allow(non_snake_case)]


pub mod Cacls {
    pub mod AAA{
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
        pub fn read_csv(Input:String) -> std::io::Result<Vec<String>> {
            let mut F = File::open(Input)?;
            let mut data = String::new();
            F.read_to_string(&mut data)?;
            let _ = data.retain(|c| c != '\r');
            let mut Vec :Vec<String> = Vec::new();
            let lines= data.lines();
            for Line in lines {
                Vec = Line.split(',').map(|T| T.to_string()).collect();//vec!( Vec![],Vec![]......) <------- ????
                println!("{:?}", Vec);
            }
            Ok(Vec)
        }
    }
    pub mod K_Series {
        pub struct KNN {
            //uhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
        }
    }
}
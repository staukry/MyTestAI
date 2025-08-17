

#[allow(non_snake_case)]
pub mod A {
    pub mod data{
        use std::fs::File;
        use std::io::Read;

        pub fn read_csv(Input:String) -> std::io::Result<()> {
            let mut F = File::open(Input)?;
            let mut data = String::new();
            F.read_to_string(&mut data)?;

            let _ = data.retain(|c| c != '\r');
            // let Targets: &[_] = &['\r'];
            // let mut buffers = data.trim_matches(Targets);
            println!("{:?}",data);
            Ok(())
        }
    }
    // pub mod basic {
    //     // pub fn add(a:f32 ,b:f32) -> f32{
    //     //     return a + b;
    //     // }
    //     // pub fn minus(a:f32,b:f32) -> f32 {
    //     //     return a-b;
    //     // }
    //     // pub fn multi(a:f32,b:f32) -> f32 {
    //     //     return a*b;
    //     // }
    //     // pub fn divide(a:f32,b:f32) -> f32 {
    //     //     return a/b;
    //     // }

    // }
}
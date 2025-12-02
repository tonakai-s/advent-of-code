pub mod problems;

pub mod common {
    use std::{fs::File, io::Read};

    // TODO: Can I choose the path based on the running profile?
    pub fn read_ex_inp(n: u8, p: u8) -> String {
        let path = format!("inputs/problem{n}/example.inp{p}.txt");
        let mut f = File::open(&path).unwrap();
        let mut content = String::new();
        let _ = f.read_to_string(&mut content);
        content
    }

    pub fn read_inp(n: u8, p: u8) -> String {
        let path = format!("inputs/problem{n}/inp{p}.txt");
        let mut f = File::open(&path).unwrap();
        let mut content = String::new();
        let _ = f.read_to_string(&mut content);
        content
    }
}

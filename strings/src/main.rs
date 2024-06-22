// This is about strings manipulation in rust
// tasks:
// - converting a string to Vec<String>
// - playing with str and String
// - reading file and printin its content to stdout
//

use std::io::Error;
use std::str::FromStr;

use std::fs;

fn main() {
    println!("Hello, world!");
    let myVec = to_vec("This is someyhing i like".into());
    print!("the string is: {:?}", myVec);
    print!("the contents of the file is: {:?}", read_file("read.txt"))
}

fn to_vec(data: &'static str) -> Vec<&'static str> {
    let res: Vec<&'static str> = data.split_whitespace().collect();
    return res;
}

fn to_vec_string(data: &'static str) -> Vec<String> {
    let res = String::from_str(data).expect("i'm getting it");
    return res.split_whitespace().map(|v| v.to_string()).collect();
}

fn read_file(fname: &'static str) -> Result<String, String> {
    print!(
        "the current working dir is: {:?}",
        fs::read_dir(".").unwrap()
    );
    let f = fs::read_to_string(fname);
    if f.is_err() {
        return Err(String::from("no available rows"));
    }
    return Ok(f.unwrap());
}

use crate::array::array::{FixedArray};
use std::string::String;

mod array;
fn main() {
    let mut fixed_arr = FixedArray::<String>::new(20).unwrap();
    fixed_arr.append(String::from("hello world!")).unwrap();
    fixed_arr.remove(0).unwrap();
    print!("{:?}", fixed_arr);
}

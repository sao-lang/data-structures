use crate::array::fixed_array::FixedArray;
use std::string::String;

mod array;
mod linked_list;
fn main() {
    let mut fixed_arr = FixedArray::<String>::new(20).unwrap();
    fixed_arr.append(String::from("hello world!")).unwrap();
    fixed_arr.remove(0).unwrap();
    print!("{:?}", fixed_arr);
}

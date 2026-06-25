mod advanced;
mod array;
mod binary_tree;
mod hash_table;
mod heap;
mod linked_list;
mod stack_queue;
mod tuple;
mod primitives;

use std::str::FromStr;

use num_bigint::BigInt;
use primitives::{Bool, Int, BigNumber, Str};

fn main() {
    // stack_queue::main();
    let mah1 = Bool::new(true) != Bool::new(false);
    let mah2 = Int::new(1) > Int::new(2);
    let bn1 = BigNumber::new(BigInt::from_str("12341234123412341298908797897897987897897").unwrap()) + BigNumber::new(BigInt::from_str("1234123412341234").unwrap());
    let str1 = Str::new("adsfas".to_string()) + Str::new("fasdfa".to_string());
    print!("{:#?}", mah1);
    print!("\n{:#?}", mah2);
    print!("\n{:#?}", bn1);
    print!("\n{:#?}", str1.value());
}

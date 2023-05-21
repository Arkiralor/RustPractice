#![allow(warnings)]

use libs::dsa::traits::UniqueElements;
mod libs;

fn main() {
    libs::try_prime_checker::try_prime_checker();
    // let num: u64 = 6;
    // let fac: u64 = libs::dsa::recursion::factorial(num);
    // println!("The factorial of {} is:\t{}", num, fac);

    // let _list: Vec<u64> = vec![1, 9, 9, 2, 0, 9, 2, 5];
    // let unique_elements: Vec<u64> = _list.unique();
    // println!("{:?}", unique_elements);
}


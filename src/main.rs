#![allow(warnings)]
mod libs;

fn main() {
    // libs::try_prime_checker::try_prime_checker();
    let num: u64 = 6;
    let fac: u64 = libs::dsa::factorial(num);
    println!("The factorial of {} is:\t{}", num, fac);
}


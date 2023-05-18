#![allow(warnings)]
use prime_checker;

pub fn try_prime_checker(){
    //! 2023-05-06 IST
    //! 
    //! ```
    //! let something = prime_checker::description(true);
    //! println!("Return for `description()`:\t {}", something);
    //! ```

    let num: u64 = 16009;
    let mut check: bool;
    let mut factors: Vec<u64>;
    let primes: Vec<u64>;
    let anti_primes: Vec<u64>;

    (check, factors) = prime_checker::check_if_anti_prime(num);
    if check == true{
        println!("\n{} is an anti-prime number.\n", num);
    }
    else {
        println!("{} is not an anti-prime number; here are its factors: {:?}\n", num, factors);
    }

    (check, factors) = prime_checker::check_if_prime(num);
    if check == true{
        println!("{} is a prime number.\n", num);
    }
    else {
        println!("{} is not a prime number; here are its factors: {:?}\n", num, factors);
    }

    primes = prime_checker::find_primes_till(num);
    println!("Prime numbers till {num}: {primes:?}\n", num=num, primes=primes);

    // anti_primes = prime_checker::find_anti_primes_till(num);
    // println!("Anti-prime numbers till {}: {:?}\n", num, anti_primes)
}

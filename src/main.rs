#![allow(warnings)]

use dotenv::dotenv;
use libs::dsa::linked_list::DoubleLinkedList;
use libs::dsa::traits::UniqueElements;
use libs::structures::schema::User;

use titlecase::titlecase;

mod libs;

fn main() {
    dotenv().ok();
    // check_env();
    try_own_package();
    // factorial_via_recursion();
    // check_unique_elements_in_list();
    // create_user();
}

fn check_unique_elements_in_list() {
    let _list: Vec<u64> = vec![1, 1, 2, 3, 5, 8, 1, 3, 2, 1, 1, 9, 9, 2, 0, 9, 2, 5];
    let unique_elements: Vec<u64> = _list.unique();
    println!(
        "The unique elements in {:?} are: {:?}",
        _list, unique_elements
    );
}

fn check_env() {
    println!("ENV: {}", dotenv().unwrap().display());
}

fn factorial_via_recursion() {
    let num: u64 = 12;
    let fac: u64 = libs::dsa::recursion::factorial(num);
    println!("The factorial of {} is:\t{}", num, fac);
}

fn try_own_package() {
    libs::try_prime_checker::try_prime_checker();
}

fn create_user() {
    let username: &str = "arkiralor";
    let email: &str = "arkiralor@gmail.com";
    let password: &str = "RandomPa$$word123";
    let incorrect_password: &str = "IncorrectPa$$word@123";
    let date_of_birth: &str = "1992-09-25";
    let gender: &str = "other";

    let user = User::create(
        Some(username),
        Some(email),
        Some(password),
        Some(date_of_birth),
        Some(titlecase(gender).as_str()),
    );
    println!("{}", user);
    println!("User (serialized):\t{}", user.serialize());
    println!("Age:\t{:?} years", user.age());

    println!(
        "Password Verification (TRUE): {}",
        user.verify_password(password)
    );
    println!(
        "Password Verification (FALSE): {}",
        user.verify_password(incorrect_password)
    );
}

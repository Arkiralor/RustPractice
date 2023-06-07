#![allow(warnings)]

use dotenv::dotenv;
use libs::dsa::linked_list::DoubleLinkedList;
use libs::dsa::traits::UniqueElements;
use libs::structures::schema::User;

use titlecase::titlecase;

mod libs;

fn main() {
    dotenv().ok();
    // println!("ENV: {}", dotenv().unwrap().display());
    // libs::try_prime_checker::try_prime_checker();
    // let num: u64 = 12;
    // let fac: u64 = libs::dsa::recursion::factorial(num);
    // println!("The factorial of {} is:\t{}", num, fac);

    // let _list: Vec<u64> = vec![1, 1, 2, 3, 5, 8, 1, 3, 2, 1, 1, 9, 9, 2, 0, 9, 2, 5];
    // let unique_elements: Vec<u64> = _list.unique();
    // println!(
    //     "The unique elements in {:?} are: {:?}",
    //     _list, unique_elements
    // );

    // let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();

    // Add elements to the front of the list
    // list.push_front(3);
    // list.push_front(7);
    // list.push_front(10);
    // list.push_back(19);
    // list.push_back(23);
    // // list.print_values();
    // list.pop_back();
    // // list.pop_front();
    // list.print_values();

    let username: &str = "arkiralor";
    let email: &str = "arkiralor@gmail.com";
    let password: &str = "RandomPa$$word123";
    let incorrect_password: &str = "IncorrectPa$$word@123";
    let date_of_birth: &str = "1992-09-25";
    let gender: &str = "male";

    let user = User::create(
        Some(username),
        Some(email),
        Some(password),
        Some(date_of_birth),
        Some(titlecase(gender).as_str()),
    );
    // println!("{}", user);
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

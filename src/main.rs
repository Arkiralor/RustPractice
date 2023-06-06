#![allow(warnings)]

use libs::dsa::linked_list::DoubleLinkedList;
use libs::dsa::traits::UniqueElements;
mod libs;

fn main() {
    // libs::try_prime_checker::try_prime_checker();
    // let num: u64 = 6;
    // let fac: u64 = libs::dsa::recursion::factorial(num);
    // println!("The factorial of {} is:\t{}", num, fac);

    // let _list: Vec<u64> = vec![1, 9, 9, 2, 0, 9, 2, 5];
    // let _list: Vec<u64> = vec![1, 1, 2, 3, 5, 8, 1, 3, 2, 1];
    // let unique_elements: Vec<u64> = _list.unique();
    // println!("The unique elements in {:?} are: {:?}", _list, unique_elements);

    let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();

    // Add elements to the front of the list
    list.push_front(3);
    list.push_front(7);
    list.push_front(10);
    list.push_back(19);
    list.push_back(23);
    // list.print_values();
    list.pop_back();
    // list.pop_front();
    list.print_values();
}

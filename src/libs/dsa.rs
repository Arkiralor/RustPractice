
pub fn factorial(num: u64) -> u64 {
    if num == 1 as u64 {
        return 1;
    }
    else {
        return num*factorial(num-1);
    }
}
pub fn factorial(num: u64) -> u64 {
    //! Function to find the factorial of a given unsigned, 64-bit number.
    if num == 1 as u64 {
        return 1;
    }
    else {
        return num*factorial(num-1);
    }
}
fn main() {
    println!("{}", fibonacci(80));
}

fn fibonacci(nth: u64) -> u64 {
    let mut memo = vec![0; nth as usize + 1];
    fn fib(nth: u64, memo: &mut Vec<u64>) -> u64 {
        if nth == 0 {
            return 0;
        } else if nth == 1 {
            return 1;
        } else if memo[nth as usize] != 0 {
            return memo[nth as usize];
        } else {
            memo[nth as usize] = fib(nth - 1, memo) + fib(nth - 2, memo);
            return memo[nth as usize];
        }
    }
    fib(nth, &mut memo)
}

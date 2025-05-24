fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0; // it should panic here instead
    } else if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    } else {
        return fib(n -1) + fib(n - 2);
    }
}

fn main() {
    let n = 7;
    dbg!(fib(n));
}

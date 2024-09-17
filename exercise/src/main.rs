/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut len = 0;
    while n > 1 {
        len += 1;
        if n % 2 == 0 {
            n = n / 2
        } else {
            n = 3 * n + 1
        }
    }
    len
}

fn main() {
    for n in 1..5 {
        let result: u32 = collatz_length(n);
        println!("len for n={}: {:?}", n, result)
    }
}

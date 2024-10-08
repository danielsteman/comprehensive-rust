mod collatz_length;
mod expression_evaluation;
mod geometry;
mod transpose;

fn main() {
    for n in 1..5 {
        let result: u32 = collatz_length::collatz_length(n);
        println!("len for n={}: {:?}", n, result)
    }
}

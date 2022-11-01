use flechs::timeline::lower_bound;

fn main() {
    let source = vec![1, 2, 4, 8, 16, 32, 64, 128];
    lower_bound(&source, &3);
    println!("Hello, world!");
}

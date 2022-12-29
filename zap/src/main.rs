use zap_add::add;
use zap_reduce::reduce;

fn main() {
    println!("Hello, world!");

    let r1 = add(12, 5);
    println!("r1 = {}", r1);

    let r2 = reduce(12, 5);
    println!("r2 = {}", r2);
}

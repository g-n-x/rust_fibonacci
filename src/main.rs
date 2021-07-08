use rand::Rng;

fn main() {
    let ceiling: u32 = rand::thread_rng().gen_range(4000..400000);
    println!("print fib numbers below {}", ceiling);
    fib(ceiling);
}

fn fib(ceil: u32) {
    let (mut a, mut b, mut c): (u32, u32, u32) = (0, 1, 0);
    while c <= ceil {
        println!("{}", a);
        c = a + b;
        a = b;
        b = c;
    }
}

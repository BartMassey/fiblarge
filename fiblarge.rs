use num::BigUint;

fn main() {
    let n: u32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut a_buf = Vec::with_capacity(3 * n as usize);
    a_buf.push(1);
    let mut a = BigUint::new(a_buf);
    let mut b_buf = Vec::with_capacity(3 * n as usize);
    b_buf.push(1);
    let mut b = BigUint::new(b_buf);
    for _ in 0 .. n / 2 {
        a += &b;
        b += &a;
    }
    println!("{}", b);
}

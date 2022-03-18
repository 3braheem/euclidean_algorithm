fn main() {
    println!("Enter an integer:");
    let mut a = String::new();
    std::io::stdin()
        .read_line(&mut a)
        .unwrap();
    println!("Enter another integer:");
    let mut b = String::new();
    std::io::stdin()
        .read_line(&mut b)
        .unwrap();
    let a = a.trim().parse::<i32>().unwrap();
    let b = b.trim().parse::<i32>().unwrap();
    println!("The greatest common divisor of {a} and {b} is: {}", gcd(a, b));
}

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        return x;
    } 
    return gcd(y, x % y);
}

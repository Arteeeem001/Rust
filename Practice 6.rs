fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let num1 = 48;
    let num2 = 18;
    let result = gcd(num1, num2);
    println!("Найбільший спільний дільник {} і {}: {}", num1, num2, result);
}
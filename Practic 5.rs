const WIDTH: usize = 28;  // Ширина конверта
const HEIGHT: usize = 13; // Висота конверта

fn main() {
    let mut output = String::new();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 {
                output.push('*');
            } else if j == 0 || j == WIDTH - 1 {
                output.push('*');
            } else if i == HEIGHT / 2 && j == WIDTH / 2 {
                output.push('*');
            } else if i == (HEIGHT / 2) - 1 && j == (WIDTH / 2) - 1 {
                output.push('*');
            } else if i == (HEIGHT / 2) - 1 && j == (WIDTH / 2) + 1 {
                output.push('*');
            } else if i == (HEIGHT / 2) + 1 && j == (WIDTH / 2) - 1 {
                output.push('*');
            } else if i == (HEIGHT / 2) + 1 && j == (WIDTH / 2) + 1 {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    println!("{}", output);
}

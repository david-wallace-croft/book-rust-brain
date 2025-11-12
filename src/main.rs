use std::io::stdin;

fn main() {
    println!("What is 3 + 2? Type your answer and press enter.");

    let mut input: String = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Unable to read standard input");

    if input == "5" {
        println!("Correct!");
    } else {
        println!("Incorrect!");
    }
}

fn p1() {
    const F32: f32 = 3.4028236;

    println!("{F32}");

    let b: u32 = F32.to_bits();

    println!("{b:032b}");

    println!("sign: {:01b}", b >> 32 - 1);

    println!("exponent: {:08b}", (b >> 32 - 9) & 0b1111_1111);

    println!("exponent: {}", (b >> 32 - 9) & 0b1111_1111);

    println!("mantissa: {:023b}", b & 0b0111_1111_1111_1111_1111_1111);

    println!("mantissa: {}", b & 0b0111_1111_1111_1111_1111_1111);
}

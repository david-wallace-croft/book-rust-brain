fn main() {
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

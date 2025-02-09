fn main() {
    let a: u8 = 0b1100_1100; // 204 in decimal
    let b: u8 = 0b1010_1010; // 170 in decimal

    // Bitwise AND
    let and_result = a & b; // 0b1000_1000 (136 in decimal)

    // Bitwise OR
    let or_result = a | b; // 0b1110_1110 (238 in decimal)

    // Bitwise XOR
    let xor_result = a ^ b; // 0b0110_0110 (102 in decimal)

    // Bitwise NOT
    let not_result = !a; // Inverts bits of a (0b0011_0011, which is 51 in decimal)

    // Left Shift
    let left_shift_result = a << 2; // 0b0011_0010 (51 in decimal)

    // Right Shift
    let right_shift_result = a >> 2; // 0b0011_0011 (51 in decimal)

    println!("AND: {:08b}", and_result);
    println!("OR: {:08b}", or_result);
    println!("XOR: {:08b}", xor_result);
    println!("NOT: {:08b}", not_result);
    println!("Left Shift: {:08b}", left_shift_result);
    println!("Right Shift: {:08b}", right_shift_result);
    println!("AND: {}", and_result);
    println!("OR: {}", or_result);
    println!("XOR: {}", xor_result);
    println!("NOT: {}", not_result);
    println!("Left Shift: {}", left_shift_result);
    println!("Right Shift: {}", right_shift_result);
}

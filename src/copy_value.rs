fn main() {
    println!("Bank in Rust -- Ownership, Borrowing");

    let num = 5;
    let other_num = num; // this type of value will not be moved, but copied from the num variable

    println!("{} {}", num, other_num);

}

/* All numbers (EX: i32, u32, f32), bool (true/false), char (single characters), Arrays (If everything inside is copyable), Tuples (If everything inside is copyable), References (both readable and writable) - Are all types with values that are copied instead of moved, so ownership doesn't work the same

*/
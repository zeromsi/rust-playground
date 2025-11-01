mod chapter_one;
fn main() {
    println!("Hello, world!");
    // chapter_one::immutability::immutability(); // remove comment to check error
    chapter_one::mutability::mutability();
    chapter_one::number::integer::print_int();
    chapter_one::number::float::print_float();
}


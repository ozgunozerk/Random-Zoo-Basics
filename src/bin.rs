use hardprime::demo;
use hardprime::find_largest_prime;

// 16-bit prime -> 65521
// 32-bit prime -> 4294967197
// 40-bit prime -> 1099511627689
// 48-bit prime -> 281474976710597
// 56-bit prime -> 72057594037927909

pub fn main() {
    demo();
    println!("{}", find_largest_prime(40).0);
    println!("{}", find_largest_prime(56).0);
}

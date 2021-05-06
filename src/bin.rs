use hardprime::demo;
use hardprime::find_largest_prime;

// 16-bit prime -> 65521
// 32-bit prime -> 4294967197
// 48-bit prime -> 281474976710597

pub fn main() {
    demo();
    find_largest_prime(64);
}

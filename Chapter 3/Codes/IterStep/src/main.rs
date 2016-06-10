fn main() {
    // for i in std::iter::range_step(5i32, 0, -1) { // std::iter::range_step is deprecated
    for i in (1..6).rev() {
    	print!("{} - ", i);
    }
}
// 5 - 4 - 3 - 2 - 1 -
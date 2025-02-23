const NUMS: [&str; 10] = [
    "Ten", "Nine", "Eight", "Seven", "Six", "Five", "Four", "Three", "Two", "One",
];
fn main() {
    println!("{:?}", NUMS.into_iter().rev().collect::<Vec<&str>>());
}

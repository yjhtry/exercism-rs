fn main() {
    let name = "余俊浩";

    println!("{}", name.len());
    println!("{}", name.chars().count());
    println!("{:?}", b"hello");

    for c in name.chars() {
        println!("{}", c);
    }
}

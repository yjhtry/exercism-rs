use time::Duration;
use time::macros::datetime;

fn main() {
    let d = datetime!(2019-01-01 0:00);

    println!("{:?}", d + Duration::seconds(1_000_000_000));
}

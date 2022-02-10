const GIFTS:[&str;12] = [
    "partridge in a pear tree",
    "turtle doves",
    "french hens",
    "calling birds",
    "golden rings",
    "geese a-laying",
    "swans a-swimming",
    "maids a-milking",
    "ladies dancing",
    "lords a-leaping",
    "pipers piping",
    "drummers drumming"
];

fn main() {
    for day in 0..12 {
        println!("On the {}th day of christmas my true love gave to me", day+1);
        for gift in (0..day+1).rev() {
            println!("{} {}", gift+1, GIFTS[gift]);
        }
    }
    println!("Hello, world!");
}

// With some help from
// https://codereview.stackexchange.com/questions/235271/12-days-of-christmas-in-rust

const GIFTS: [&str; 12] = [
    "twelve drummers drumming",
    "eleven pipers piping",
    "ten lords a-leaping",
    "nine ladies dancing",
    "eight maids a-milking",
    "seven wans a-swimming",
    "six geese a-laying",
    "five gold rings",
    "four calling birds",
    "three French hens",
    "two turtle doves",
    "a partridge in a pear tree",
];

fn with_suffix(n: usize) -> String {
    let res = match n {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    format!("{}{}", n, res)
}

fn gen_gifts(day: usize) -> String {
    let mut result: String = String::from("");
    let ignored = 12 - day;
    for (i, gift) in GIFTS.iter().skip(ignored).enumerate() {
        if i + ignored == GIFTS.len() - 1 && i > 1 {
            result.push_str(&format!("\n  and {}", gift));
        } else {
            result.push_str(&format!("\n  {}", gift));
        }
    }
    result
}

fn main() {
    for i in 1..=12 {
        println!(
            "On the {} day of Christmas, my true love gave to me...{}",
            with_suffix(i),
            gen_gifts(i)
        );
    }
}

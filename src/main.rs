struct Day {
    num: usize,
    gift: String,
}

impl Day {
    fn new(num: usize, gift: String) -> Day {
        Day { num, gift }
    }

    fn lyric(&self) -> String {
        format!(
            "On the {} day of Christmas, my true love gave to me...",
            nth(&self)
        )
    }
}

fn nth(day: &Day) -> String {
    let suffix = match day.num {
        1 => String::from("st"),
        2 => String::from("nd"),
        3 => String::from("rd"),
        _ => String::from("th"),
    };
    format!("{}{}", day.num, suffix)
}

fn print_song(days: [Day; 12]) {
    for n in 1..=12 {
        for (i, day) in days.iter().skip(12 - n).enumerate() {
            if i == 0 {
                println!("\n{}", day.lyric());
            }
            if day.num == 1 && n != 1 {
                println!("  and {}", day.gift);
            } else {
                println!("  {}", day.gift);
            }
        }
    }
}

fn main() {
    print_song([
        Day::new(12, String::from("twelve drummers drumming")),
        Day::new(11, String::from("eleven pipers piping")),
        Day::new(10, String::from("ten lords a-leaping")),
        Day::new(9, String::from("nine ladies dancing")),
        Day::new(8, String::from("eight maids a-milking")),
        Day::new(7, String::from("seven swans a-swimming")),
        Day::new(6, String::from("six geese a-laying")),
        Day::new(5, String::from("five gold rings")),
        Day::new(4, String::from("four calling birds")),
        Day::new(3, String::from("three French hens")),
        Day::new(2, String::from("two turtle doves")),
        Day::new(1, String::from("a partridge in a pear tree")),
    ]);
}

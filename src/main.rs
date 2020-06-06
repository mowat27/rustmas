fn main() {
    let gifts = [
        ("first", "a partridge in a pear tree"),
        ("second", "two turtle doves"),
        ("third", "three French hens"),
        ("fourth", "four calling birds"),
        ("fifth", "five gold rings"),
        ("sixth", "six geese a-laying"),
        ("seventh", "seven wans a-swimming"),
        ("eighth", "eight maids a-milking"),
        ("ninth", "nine ladies dancing"),
        ("tenth", "ten lords a-leaping"),
        ("eleventh", "eleven pipers piping"),
        ("twelth", "twelve drummers drumming"),
    ];

    println!("The 12 days of Christmas");
    let mut i = 0;
    for (nth, _) in gifts.iter() {
        println!(
            "On the {} day of Christmas, my true love gave to me...",
            nth
        );
        let mut j = i;
        while j > -1 {
            let (_, gift) = gifts[j as usize];
            if i != 0 && j == 0 {
                println!("  and {}", gift);
            } else {
                println!("  {}", gift);
            }
            j -= 1;
        }
        i += 1;
    }
}

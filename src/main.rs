#![allow(non_snake_case)]
#![allow(unused)]

fn main() {
    let first_half: &str = "On the";
    let second_half: &str = "day of Christmas my true love sent to me";
    let separator: &str = "\n";
    let days: [&str; 12] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eights",
        "nines",
        "tenth",
        "eleventh",
        "twelfth",
    ];
    let words: [&str; 12] = [
        "And a partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "four calling birds,",
        "five gold rings,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,",
    ];

    for index in 0..12 {
        println!("\n{first_half} {} {second_half}", days[index]);
        // println!("{}", words[index]);
        for i_words in 0..index {
            println!("{}", words[index-i_words]);
        }
        println!("{}", words[0]);
    }

}

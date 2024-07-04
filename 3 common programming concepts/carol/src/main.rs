fn main() {
    let number = ["first", "second", "third", "fourth",
                    "fifth", "sixth", "seventh", "eighth",
                    "ninth", "tenth", "eleventh", "twlelfth"];
                    
    let lyrics = ["And a partridge in a pear tree.",
                  "Two turtle doves, ",
                  "Three French hens, ",
                  "Four calling birds, ",
                  "Five golden rings, ",
                  "Six geese a-laying, ",
                  "Seven swans a-swimming, ",
                  "Eight maids a-milking, ",
                  "Nine ladies dancing, ",
                  "Ten lords a-leaping, ",
                  "Eleven pipers piping, ",
                  "Twelve drummers drumming, "];

    for i in 0..12 {
        println!("On the {} day of Christmas,", number[i]);
        println!("my true love sent to me:");

        for j in (0..=i).rev() {
            if i == 0 {
                println!("A partridge in a pear tree.");
            } else {
                println!("{}", lyrics[j]);
            }
        }

        println!("");
    }
}

fn main() {
    let mut count = 0;
    while count < 12 {
        println!("[Verse {}]", count + 1);

        let mut day = String::new();

        day = match count + 1 {
            1 => "first".to_string(),
            2 => "second".to_string(),
            3 => "third".to_string(),
            4 => "fourth".to_string(),
            5 => "fivth".to_string(),
            6 => "sixth".to_string(),
            7 => "seventh".to_string(),
            8 => "eight".to_string(),
            9 => "ninth".to_string(),
            10 => "tenth".to_string(),
            11 => "eleventh".to_string(),
            12 => "twelfth".to_string(),
            _ => "x-th".to_string(),
        };

        println!("On the {day} day of Christmas, my true love sent to me");
        if count >= 11 {
            println!("Twelve drummers drumming");
        }
        if count >= 10 {
            println!("Eleven pipers piping");
        }
        if count >= 9 {
            println!("Ten lords a-leaping");
        }
        if count >= 8 {
            println!("Nine ladies dancing");
        }
        if count >= 7 {
            println!("Eight maids a-milking");
        }
        if count >= 6 {
            println!("Seven swans a-swimming");
        }
        if count >= 5 {
            println!("Six geese a laying");
        }
        if count >= 4 {
            println!("Five golden rings");
        }
        if count >= 3 {
            println!("Four calling birds");
        }
        if count >= 2 {
            println!("Three french hens");
        }
        if count >= 1 {
            println!("Two turtle doves and");
        }
        println!("A partridge in a pear tree");
        count += 1;
    }
}

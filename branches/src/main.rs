fn main() {
    
    // project 1: Christmass carol
    let mut christmass = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five gold rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtledoves",
        "And a partridge in a pear tree"
    ];
    christmass.reverse();
    let days = 12;

    for day in 1..days+1 {
        println!("On the {day} day of Christmas, my true love sent to me");
        for pos in 0..day {
            println!("{}", christmass[pos]);
        }
        println!("                        ")
    }

}

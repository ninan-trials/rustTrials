fn main() {
    let days_song_tuple = [
        ("first", "A Partridge in a Pear Tree,"),
        ("second", "Two Turtle Doves,"),
        ("third", "Three French Hens,"),
        ("fourth", "Four Calling Birds,"),
        ("fifth", "Five Golden Rings,"),
        ("sixth", "Six Geese a Laying,"),
        ("seventh", "Seven Swans a Swimming,"),
        ("eight", "Eight Maids a Milking,"),
        ("ninth", "Nine Ladies Dancing,"),
        ("tenth", "Ten Lords a Leaping,"),
        ("eleventh", "Eleven Pipers Piping,"),
        ("twelfth", "Twelve Drummers Drumming,")
    ];

    for (index, day_num) in days_song_tuple.iter().enumerate() {
        println!("On the {} day of Christmas,", day_num.0);
        println!("My true love sent to me:");
        if index > 0 {
            let mut day = index;
            while day > 0 {
                println!("{}", days_song_tuple[day].1);
                day = day - 1;
            }
            println!("And {}", days_song_tuple[day].1);
        } else {
            print!("{}", day_num.1);
        }
        println!("\n");
    }
}

fn main() {
    let days = [
        "On the first day of Christmas,",
        "On the second day of Christmas,",
        "On the third day of Christmas,",
        "On the fourth day of Christmas,",
        "On the fifth day of Christmas,",
        "On the sixth day of Christmas,",
        "On the seventh day of Christmas,",
        "On the eighth day of Christmas,",
        "On the ninth day of Christmas,",
        "On the tenth day of Christmas,",
        "On the eleventh day of Christmas,",
        "On the twelfth day of Christmas,",
    ];

    let deeds = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    let mut day = 1;
    let mut count = 1;

        for element in days {
            println!("{}", element);
            println!("my true love gave to me");

            while day > 0 {
                for _ in (0..day).rev() {
                    if count > 1 && day == 1 {
                        println!("And a partridge in a pear tree.");
                    } else {

                        println!("{}", deeds[day - 1]);
                    }
                    
                    day -= 1;
                }
            }
            count += 1;
            day = count;
        }

}

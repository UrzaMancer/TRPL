fn main() {
    println!("The twelve days of Christmas! Let's begin.");

    let mut n = 0;
    let number_words = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let verses = [
                  "A partridge in a pear tree.",
                  "Two turtle doves, and",
                  "Three french hens,",
                  "Four calling birds,",
                  "Five golden rings,",
                  "Six geese a-laying,",
                  "Seven swans a-swimming,",
                  "Eight maids a-milking,",
                  "Nine ladies dancing,",
                  "Ten lords a-leaping,",
                  "Eleven pipers piping,",
                  "Twelve drummers drumming,"];

    while n < 12 {
        println!("On the {} day of Christmas, my true love sent to me", number_words[n]);
        n=n+1;
        for c in (0..n).rev() {
            println!("{}", verses[c]);
        }
    }
}

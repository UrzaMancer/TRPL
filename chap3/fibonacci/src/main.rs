use std::io;
fn main() {
    println!("Welcome to jechamt 's Fibonacci generator!");
    println!("We will generate the nth Fibonacci number. Please enter n < 46:");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Expected an integer as input, but instead got: {}", n),
    };

    fn generate_fibonacheese(n: u32) {
        if n > 45 {
            println!("You entered n = {}, which is too big for me. We'll use n=10 instead", n);
            generate_fibonacheese(10);
            return;
        }
        println!("Ok! Let's begin...");
        let mut next: u32;
        let mut last = 1;
        let mut this = 1;
        let mut counter = 0;
        let mut output = String::new();
        output.push_str("1 1 ");
        while counter < n {
            next = this + last;
            last = this;
            this = next;
            output.push_str(&next.to_string());
            output.push_str(" ");
            println!("{}: {} ",counter+1, &output);
            counter+=1;
        }
    }

    generate_fibonacheese(n);
}

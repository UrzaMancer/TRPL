use std::io;


fn main() {
    println!("Please enter a temperature");
    let mut input_temp = String::new();

    io::stdin().read_line(&mut input_temp)
        .expect("Failed to read line");

    let input_temp: i32 = match input_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Expected an integer as input, but instead got: {}", input_temp),
    };

    fn celsius_to_fahrenheit(c_temp: i32) {
        let f_temp = c_temp * 9 / 5 + 32;
        println!("{} degrees Celsius is {} degrees Fahrenheit", c_temp, f_temp);
    }

    fn fahrenheit_to_celsius(f_temp: i32) {
        let c_temp = (f_temp - 32) * 5 / 9;
        println!("{} degrees Fahrenheit is {} degrees Celsius", f_temp, c_temp);
    }

    celsius_to_fahrenheit(input_temp);
    fahrenheit_to_celsius(input_temp);

}

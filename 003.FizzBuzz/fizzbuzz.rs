fn main() {
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            print!("FizzBuzz ");
        }
        else if i % 3 == 0 {
            print!("Fizz     ");
        }
        else if i % 5 == 0 {
            print!("Buzz     ");
        }
        else {
            print!("XXXX     ");
        }

        if i % 10 == 0{
            println!();
        }
    }
}
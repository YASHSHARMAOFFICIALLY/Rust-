use std::io;


pub fn fizz_buzz(num:u32)->String{
    if num % 3 == 0 {
        return "Fizz".to_string()

    }else if num % 5 == 0 {
        return "Buzz".to_string()
    }else if num % 3 == 0 && num % 5 == 0 {
        return "FizzBuzz".to_string()
    }else  {
        num.to_string()
    } 
}

fn main(){
    println!("enter the number");


    let mut number = String::new();
        io::stdin()
        .read_line(&mut number)
        .expect("failed to read line");

    let number = number.trim().parse().expect("failed to read line");

    let result = fizz_buzz(number);
    println!("{result}")
}
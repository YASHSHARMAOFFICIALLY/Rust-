use std::io;
fn main(){
    println!("Welcome to Fibo");

    println!("Enter the value you want till fibo");
    let mut num = String::new();
            io::stdin()
            .read_line(&mut num)
            .expect("Failed to read value");
    
    let num = num.trim().parse().expect("please enter a valid value");

    if num <= 1 {
        return 0;
    }

    let mut a = 0;
    let mut b = 1;

    if num == 1 {
        return a;
    };
    else if num == 2 {
        return b;
    }else {
        let next = a +b;
        b = a;
        b = next;
    }

}
use std::io;
pub fn is_even(n:i32)->bool{
    if n % 2 == 0 {
        return true;
    }else {
        return false;
    }
}

fn main(){
    println!("Enter the number");

    let mut num:String = String::new();
            io::stdin()
        .read_line(&mut num)
         .expect("Failed to read value");
    
    let num = num.trim().parse().expect("Failed to read the input");

    let even = is_even(num);
    println!("{even}");
}
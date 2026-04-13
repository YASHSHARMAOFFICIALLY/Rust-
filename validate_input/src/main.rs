use std::io;

fn function_validate(age:i32,email:&str)-> Result<(),String>{
    if age < 0 || age > 120 {
        return Err("Age cant be these".to_string());
    }
        if !email.contains("@") {
        return Err("Invalid email format".to_string());
    }

    else {
          Ok(())
    }
}

fn main(){
    println!("Welcome to input validate");

    println!("Enter the age you want to validate");

    let mut age = String::new();
        io::stdin()
        .read_line(&mut age)
        .expect("failed to read line");

    let age:i32= age.trim().parse().expect("failed to read");

    println!("Enter the email you want");

    let mut email = String::new();
        io::stdin()
        .read_line(&mut email)
        .expect("failed to read line");
    
    let email:String = email.trim().parse().expect("Failed to get email");

    let validate = function_validate(age,&email);
    println!("{:?}",validate);
   
}
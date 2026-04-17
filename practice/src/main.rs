//Vecotr 

//Vector bacic 

// fn main(){
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);

//     println!("{:?}",vec)
// }


//vector plain text even number only 


// fn odd_value(v:&mut Vec<i32>){
//     let mut i = 0;
//     while i < v.len(){
//         if v[i] % 2 == 0 {
//             v.remove(i);
//         }else{
//             i += 1;
//         }
//     }
// }

// fn evenn_value(v:&Vec<i32>)-> Vec<i32>{
//     let mut new_vec = Vec::new();
//     for &value in v {
//         if value % 2 == 0 {
//             new_vec.push(value)
//         }
//     }
//     return new_vec;
// }


// fn main(){
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
    
//     let result = evenn_value(& vec);
//     odd_value(&mut vec);
//     println!("{:?}",result);
//     println!("{:?}",vec);
// }



// fn main(){
//     let numbers = vec![1,2,3];
//     for number in numbers {
//         println!("{}",number);
//     }
// }


// fn main(){
//     let numbers: Vec<i32> = vec![1,2,3];
//     for number in numbers {
//         println!("{}",number);
//     }
// }

// use std::io;
// fn main(){
//     println!("Enter the size of Array");
//     let mut input = String::new();
//         io::stdin()
//         .read_line(&mut input)
//         .unwrap();

//     // let input:<u32>  = input.trim().parse().unwrap();

//     let vec:Vec<i32>= input
//         .trim()
//         .split_whitespace()
//         .map(|x| x.parse().unwrap())
//         .collect();

//     println!("{:?}",vec)
        
// }


// use std::collections::HashMap;
// fn main(){
//     let mut user = HashMap::new();
//     user.insert(String::from("harkirat"),32);
//     user.insert(String::from("raman"),32);

//     let first_user_age = user.get("harkirat");
//     println!("{:?}",first_user_age);
// }


// use std::collections::HashMap;

// fn group_value_by_keys(vec:Vec<(String,i32)>) -> HashMap<String,i32>{
//     let mut hm = HashMap::new();
//     for(key,value)in vec {
//         hm.insert(key,value);
//     }
//     return hm;
// }

// fn main(){
//     let input_vec = vec![(String::from("harikart"),22),(String::from("raman"),32)];
//     let hm = group_value_by_keys(input_vec);

//     println!("{:?}",hm)
// }


// fn main(){
//     let mut v1 = vec![1,2,3];
//     let v1_iter = v1.iter_mut();
//     for val in v1_iter{
//          *val = *val + 1
// }
//     println!("{:?}",v1)
// }


fn main(){
    let mut v1 = vec![1,2,3];
    let mut v1_iter = v1.iter_mut();

    while let Some(val) = v1_iter.next(){
        println!("{}",val)
    }
    println!("{:?}",v1)
}
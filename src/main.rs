use std::io;
fn main() {
     let mut operation:String = String::new();
     let mut num1:String = String::new();
     let mut num2:String = String::new();

     println!("enter the operation");
     io::stdin().read_line(&mut operation).expect("failed to take input");
     println!("enter the first number");
     io::stdin().read_line(&mut num1).expect("failed to take input");
     println!("enter the second  number");
     io::stdin().read_line(&mut num2).expect("failed to take input");
     
     let num1:i32 = num1.trim().parse().expect("something went wrong");
     let num2:i32 = num2.trim().parse().expect("something went wrong");
     let operation = operation.trim(); 


     let result = match operation {
        "+" => num1 +num2,
        "*" => num1*num2,
        "-" => num1 - num2,
        "/" => num1 / num2,
        _=> {
            println!("invalid operation");
            return 
        }
     };
     println!("{}", result);


} 
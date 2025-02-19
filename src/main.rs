/* fn main() {
    #[derive(Debug)]
    struct Employee {
        name:String,
        age:u32,
        salary:f64
    }
    impl Employee {
        fn get_details(&self) {
            println!("the employe name is {} and age is {} and earn {}",self.name,self.age,self.salary);
        }
        fn increase_salary(&mut self,amount:f64){
            self.salary += amount;
            println!("salary is increased with {}",self.salary);
        }
    }
    let mut anjali:Employee = Employee{
        name: String::from("Anjali"),
        age:22,
        salary:5000.0
    };
    println!("{:?}" , anjali);
    anjali.get_details();
    anjali.increase_salary(40000.0);
} */

/* Temperature Converter */
use std::io;
fn main() {
    println!("welcome to price converter💫");
    let mut celcius: String = String::new();
    println!("what Temperature you want to convert inter below ");
    io::stdin().read_line(&mut celcius).expect("invalid input");
    let celcius:f64 = celcius.trim().parse().expect("not converted");
    let fahrenheit = celcius *1.8 + 32.0;
    println!("fahrenheit {}" , fahrenheit);

}
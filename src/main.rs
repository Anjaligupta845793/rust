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
/* use std::io;
fn main() {
    println!("welcome to price converter💫");
    let mut celcius: String = String::new();
    println!("what Temperature you want to convert inter below ");
    io::stdin().read_line(&mut celcius).expect("invalid input");
    let celcius:f64 = celcius.trim().parse().expect("not converted");
    let fahrenheit = celcius *1.8 + 32.0;
    println!("fahrenheit {}" , fahrenheit);

} */

use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Self {
        Self {
            description,
            completed: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
            return;
        }
        for (i, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "[✔]" } else { "[ ]" };
            println!("{} {} {}", i, status, task.description);
        }
    }

    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        } else {
            println!("Invalid index!");
        }
    }

    fn save_to_file(&self) {
        let json = serde_json::to_string(&self.tasks).unwrap();
        fs::write("tasks.json", json).unwrap();
    }

    fn load_from_file(&mut self) {
        if let Ok(json) = fs::read_to_string("tasks.json") {
            self.tasks = serde_json::from_str(&json).unwrap_or_else(|_| Vec::new());
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    todo_list.load_from_file();

    loop {
        println!("\n1. Add Task\n2. List Tasks\n3. Remove Task\n4. Save & Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();
                todo_list.add_task(desc.trim().to_string());
            }
            "2" => todo_list.list_tasks(),
            "3" => {
                print!("Enter task index to remove: ");
                io::stdout().flush().unwrap();
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();
                if let Ok(i) = index.trim().parse::<usize>() {
                    todo_list.remove_task(i);
                }
            }
            "4" => {
                todo_list.save_to_file();
                println!("Tasks saved. Exiting...");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

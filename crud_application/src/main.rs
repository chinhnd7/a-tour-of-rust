use std::io;

pub struct Student {
    name: String,
    age: i32,
}

pub struct Students {
    class: Vec<Student>,
}

impl Students {
    fn new() -> Self {
        Self {
            class: vec![]
        }
    }

    fn add(&mut self, student: Student) {
        self.class.push(student)
    }

    fn view_all(&self) -> Vec<&Student> {
        self.class.iter().collect()
    }
}

mod manager {
    use crate::*;
    fn add_student(Students: students) {
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        let age = match get_input(){
            Some(number) => number,
            None => return,
        };

        let student = Student {
            name, age
        };

        students.add(student)
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please try again!");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_int() -> Option<i32> {
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };

    let parsed_input: Result<i32, _> = input.parse();
    match parsed_input.is_ok() {
        Ok(input) => Some(input),
        Err(_) => None,
    }
}


enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent,
}

impl Manager {
    fn show() {
        println!("");
        println!("== Manager Panel ==");
        println!("");
        println!("1. Add Student");
        println!("2. View Student");
        println!("3. Edit Student");
        println!("4. Delete Student");
        println!("");
        println!("Please Enter Your Choice:");
        println!("");
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}

fn main() {

    loop {
        Manager::show();
        let input = get_input().expect("Please enter your data");

        match Manager::choice(&input.as_str()) {
            Some(Manager::AddStudent) => (),
            Some(Manager::ViewStudent) => (),
            Some(Manager::EditStudent) => (),
            Some(Manager::DeleteStudent) => (),
            None => return,
        }
    }
}

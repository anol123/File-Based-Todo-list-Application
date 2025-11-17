use std::{fs::File, io::{self, BufReader}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task{
    id:u32,
    title: String,
    done:bool,
}

const FILE_PATH: &str ="tasks.json";

fn load_tasks()-> Vec<Task>{
    if let Ok(file)=File::open(FILE_PATH){
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
    }
    else{
        vec![]
    }
}

fn add_task(title:String){
    let mut tasks = load_tasks();
    let mut id = (tasks.len() as u32) + 1;

    let task = Task {
        id,
        title,
        done:false,
    };
    tasks.push(task);
    //save_tasks(tasks);
    println!("✅Your task is added successfully");

}

fn read_input(prompt:&str) -> String{
    println!("{prompt}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
fn main() {
    loop{
        println!("\n------ TODO MENU ------");
        println!("1. Add task\n2.List tasks\n3.Mark as done\n4.Delete task\n5.Exit\n");

        let choice = read_input("Enter your choice");
        match choice.as_str() {
            "1" =>{
                let title = read_input("Enter task title:");
                add_task(title);
            },
            _ => println!("Please enter a valid choice")
        }

    }
}

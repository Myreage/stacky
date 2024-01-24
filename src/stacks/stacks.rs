use std::fmt;

use serde::{Deserialize, Serialize};

use crate::persistence::{self, FileData};

use super::branches::Branch;

#[derive(Debug, Serialize, Deserialize)]
pub struct Stack {
    pub name: String,
    pub branches: Vec<Branch>,
}

impl Stack {
    pub fn new(name: &String) -> Stack {
        Stack {
            branches: vec![],
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Currently on stack {}\n\n", self.name)?;

        let mut line_size = 0;
        for (index, branch) in self.branches.iter().enumerate() {
            if index == 0 {
                write!(f, "{}\n", branch.name)?;
                line_size += branch.name.len();
            } else {
                let previous_element_size = self.branches.get(index - 1).unwrap().name.len();
                let number_of_spaces = line_size - previous_element_size / 2;

                for _i in 0..number_of_spaces {
                    write!(f, " ")?;
                }

                write!(f, "└──{}\n", branch.name)?;
                line_size = number_of_spaces + branch.name.len() + 3;
            }
        }

        Ok(())
    }
}

pub fn print_current_stack() -> Result<(), &'static str> {
    let file_data = match persistence::read_from_file::<FileData>("save.json") {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file"),
    };

    let current_stack = match file_data
        .stacks
        .iter()
        .find(|&s| s.name == file_data.current_stack)
    {
        Some(stack) => stack,
        None => return Err("Not currently on a stack"),
    };

    println!("{}", current_stack);

    Ok(())
}

pub fn create_stack(stack_name: &String) -> Result<(), &'static str> {
    let mut file_data = match persistence::read_from_file::<FileData>("save.json") {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file"),
    };

    if let Some(_) = file_data.stacks.iter().find(|&s| &s.name == stack_name) {
        return Err("Stack already exists");
    }

    file_data.stacks.push(Stack::new(stack_name));

    match persistence::write_to_file(&file_data, "save.json") {
        Ok(_) => {
            eprintln!("Created new stack {}", stack_name);
            Ok(())
        }
        Err(_) => Err("Failed writing to file"),
    }
}

pub fn checkout_stack(stack_name: &String) -> Result<(), &'static str> {
    let mut file_data = match persistence::read_from_file::<FileData>("save.json") {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file"),
    };

    match file_data.stacks.iter().find(|&s| &s.name == stack_name) {
        Some(_) => (),
        None => return Err("Stack not found"),
    };

    file_data.current_stack = stack_name.clone();

    match persistence::write_to_file(&file_data, "save.json") {
        Ok(_) => {
            eprintln!("On stack {}", stack_name);
            Ok(())
        }
        Err(_) => Err("Failed writing to file"),
    }
}

use std::{
    fs::File,
    io::{self, Write},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::stacks::{branches::Branch, stacks::Stack};

#[derive(Debug, Serialize, Deserialize, Default)]
struct FileData {
    pub stacks: Vec<Stack>,
    pub current_stack: String,
}

fn write_to_file<T: Serialize>(data: &T, file_path: &str) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = io::BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, data)?;
    writer.flush()?;
    Ok(())
}

fn read_from_file<T: DeserializeOwned + Default + Serialize>(file_path: &str) -> io::Result<T> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            let _ = write_to_file(&T::default(), file_path);
            File::open(file_path)?
        }
    };

    let reader = io::BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

pub fn read_current_stack_branches() -> Result<Vec<Branch>, &'static str> {
    let file_data = match read_from_file::<FileData>("save.json") {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file"),
    };

    let current_stack = &file_data.current_stack;

    let branches = &file_data
        .stacks
        .iter()
        .find(|&s| &s.name == current_stack)
        .unwrap()
        .branches;

    Ok(branches.clone())
}

pub fn save_stack(stack_name: &String) -> Result<(), String> {
    let mut file_data = match read_from_file::<FileData>("save.json") {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file".to_string()),
    };

    if let Some(_) = file_data.stacks.iter().find(|&s| &s.name == stack_name) {
        return Err("Stack already exists".to_string());
    }

    file_data.stacks.push(Stack::new(stack_name));

    match write_to_file(&file_data, "save.json") {
        Ok(_) => {
            eprintln!("Created new stack {}", stack_name);
            Ok(())
        }
        Err(_) => Err("Failed writing to file".to_string()),
    }
}

pub fn save_branch_on_current_stack(branch_name: &String) -> Result<(), String> {
    let mut file_data = match read_from_file::<FileData>("save.json") {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file".to_string()),
    };

    let current_stack_position = match file_data
        .stacks
        .iter()
        .position(|s| s.name == file_data.current_stack)
    {
        Some(position) => position,
        None => return Err("Not currently on a stack".to_string()),
    };

    let current_stack = &mut file_data.stacks[current_stack_position];

    if let Some(_) = current_stack
        .branches
        .iter()
        .find(|&s| &s.name == branch_name)
    {
        return Err("Branch already exists".to_string());
    }

    current_stack.branches.push(Branch::new(branch_name));

    match write_to_file(&file_data, "save.json") {
        Ok(_) => {
            eprintln!("Created new branch {}", branch_name);
            Ok(())
        }
        Err(_) => Err("Failed writing to file".to_string()),
    }
}

pub fn save_current_stack(stack_name: &String) -> Result<(), String> {
    let mut file_data = match read_from_file::<FileData>("save.json") {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file".to_string()),
    };

    match file_data.stacks.iter().find(|&s| &s.name == stack_name) {
        Some(_) => (),
        None => return Err("Stack not found".to_string()),
    };

    file_data.current_stack = stack_name.clone();

    match write_to_file(&file_data, "save.json") {
        Ok(_) => {
            eprintln!("On stack {}", stack_name);
            Ok(())
        }
        Err(_) => Err("Failed writing to file".to_string()),
    }
}

pub fn read_current_stack() -> Result<Stack, String> {
    let file_data = match read_from_file::<FileData>("save.json") {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file".to_string()),
    };

    let current_stack = match file_data
        .stacks
        .iter()
        .find(|&s| s.name == file_data.current_stack)
    {
        Some(stack) => stack,
        None => return Err("Not currently on a stack".to_string()),
    };

    Ok(current_stack.clone())
}

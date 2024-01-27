use std::process::Command;

use crate::persistence::{self, FileData};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Branch {
    pub name: String,
}

impl Branch {
    pub fn new(name: &String) -> Branch {
        Branch {
            name: name.to_string(),
        }
    }
}

pub fn create_branch(branch_name: &String) -> Result<(), String> {
    let mut file_data = match persistence::read_from_file::<FileData>("save.json") {
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

    let mut git_branch = Command::new("git");
    git_branch.arg("branch").arg(branch_name);

    match git_branch.output() {
        Ok(result) => {
            if !result.status.success() {
                return Err("Git branch failed".to_string());
            }
        }
        Err(_) => return Err("Git branch failed".to_string()),
    }

    current_stack.branches.push(Branch::new(branch_name));

    match persistence::write_to_file(&file_data, "save.json") {
        Ok(_) => {
            eprintln!("Created new branch {}", branch_name);
            Ok(())
        }
        Err(_) => Err("Failed writing to file".to_string()),
    }
}

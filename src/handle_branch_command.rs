use std::process::Command;

use crate::{persistence, types};

pub fn extract_branch_name(options: &[String]) -> Option<&String> {
    options.iter().find(|&o| !o.starts_with("--"))
}

pub fn extract_new_option(options: &[String]) -> Option<&String> {
    options.iter().find(|&o| o == "--new")
}

pub fn create_branch(branch_name: &String) -> Result<(), &'static str> {
    let mut file_data = match persistence::read_from_file::<types::FileData>("save.json") {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file"),
    };

    let current_stack_position = match file_data
        .stacks
        .iter()
        .position(|s| s.name == file_data.current_stack)
    {
        Some(position) => position,
        None => return Err("Not currently on a stack"),
    };

    let current_stack = &mut file_data.stacks[current_stack_position];

    if let Some(_) = current_stack
        .branches
        .iter()
        .find(|&s| &s.name == branch_name)
    {
        return Err("Branch already exists");
    }

    let mut git_branch = Command::new("git");
    git_branch.arg("branch").arg(branch_name);

    match git_branch.output() {
        Ok(result) => {
            if !result.status.success() {
                return Err("Git branch failed");
            }
        }
        Err(_) => return Err("Git branch failed"),
    }

    current_stack.branches.push(types::Branch::new(branch_name));

    match persistence::write_to_file(&file_data, "save.json") {
        Ok(_) => {
            eprintln!("Created new branch {}", branch_name);
            Ok(())
        }
        Err(_) => Err("Failed writing to file"),
    }
}

pub fn checkout_branch(branch_name: &String) -> Result<(), &'static str> {
    let mut git_checkout = Command::new("git");
    git_checkout.arg("checkout").arg(branch_name);

    match git_checkout.output() {
        Ok(result) => {
            if !result.status.success() {
                return Err("Git checkout failed");
            }
        }
        Err(_) => return Err("Git checkout failed"),
    };

    Ok(())
}

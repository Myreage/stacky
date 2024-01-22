use std::{
    io::{self, Write},
    process::Command,
};

mod handle_branch_command;
mod handle_stack_command;
mod persistence;
mod types;

pub fn handle_stack_command(options: &[String]) -> Result<(), &'static str> {
    let stack_name_result = handle_stack_command::extract_stack_name(options);

    if stack_name_result.is_none() {
        match handle_stack_command::print_current_stack() {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }

    let stack_name = stack_name_result.unwrap();

    let create_option = match handle_stack_command::extract_create_option(options) {
        Some(_) => true,
        None => false,
    };

    if create_option {
        match handle_stack_command::create_stack(stack_name) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    match handle_stack_command::checkout_stack(stack_name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn handle_branch_command(options: &[String]) -> Result<(), &'static str> {
    let branch_name = match handle_branch_command::extract_branch_name(options) {
        Some(name) => name,
        None => return Err("Missing branch name"),
    };
    let create_option = match handle_branch_command::extract_create_option(options) {
        Some(_) => true,
        None => false,
    };

    if create_option {
        match handle_branch_command::create_branch(&branch_name) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    match handle_branch_command::checkout_branch(branch_name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn handle_sync_command() -> Result<(), &'static str> {
    let file_data = match persistence::read_from_file::<types::FileData>("save.json") {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file"),
    };

    let current_stack = file_data.current_stack;

    let branches = &file_data
        .stacks
        .iter()
        .find(|&s| s.name == current_stack)
        .unwrap()
        .branches;

    // Pull main

    let mut git_checkout_main = Command::new("git");
    git_checkout_main.arg("checkout").arg("main");

    match git_checkout_main.output() {
        Ok(result) => {
            if !result.status.success() {
                io::stdout().write_all(&result.stdout).unwrap();
                io::stderr().write_all(&result.stderr).unwrap();
                return Err("Git checkout failed");
            }
        }
        Err(_) => return Err("Git checkout failed"),
    }

    let mut git_pull_main = Command::new("git");
    git_pull_main.arg("pull");

    match git_pull_main.output() {
        Ok(result) => {
            if !result.status.success() {
                io::stdout().write_all(&result.stdout).unwrap();
                io::stderr().write_all(&result.stderr).unwrap();
                return Err("Git pull main failed");
            }
        }
        Err(_) => return Err("Git pull main failed"),
    }

    println!("Bonjour saucisse");

    // Rebase and push all branches
    for (index, branch) in branches.iter().enumerate() {
        let mut git_checkout = Command::new("git");
        git_checkout.arg("checkout").arg(&branch.name);

        match git_checkout.output() {
            Ok(result) => {
                if !result.status.success() {
                    io::stdout().write_all(&result.stdout).unwrap();
                    io::stderr().write_all(&result.stderr).unwrap();
                    return Err("Git checkout failed");
                }
            }
            Err(_) => return Err("Git checkout failed"),
        }

        let rebase_branch = match index {
            0 => "main",
            _ => &branches[index - 1].name,
        };

        let mut git_rebase = Command::new("git");
        git_rebase.arg("rebase").arg(rebase_branch);

        match git_rebase.output() {
            Ok(result) => {
                if !result.status.success() {
                    io::stdout().write_all(&result.stdout).unwrap();
                    io::stderr().write_all(&result.stderr).unwrap();
                    return Err("Git rebase failed");
                }
            }
            Err(_) => return Err("Git rebase failed"),
        }

        println!("Bigoudi");

        let mut git_push = Command::new("git");
        git_push
            .arg("push")
            .arg("--set-upstream")
            .arg("origin")
            .arg(&branch.name);

        match git_push.output() {
            Ok(result) => {
                if !result.status.success() {
                    io::stdout().write_all(&result.stdout).unwrap();
                    io::stderr().write_all(&result.stderr).unwrap();
                    return Err("Git push failed");
                }
            }
            Err(_) => return Err("Git push failed"),
        }
    }

    println!("c'est super haha");

    Ok(())
}

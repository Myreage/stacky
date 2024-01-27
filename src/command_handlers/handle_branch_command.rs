use crate::{git::checkout_branch, stacks::branches::create_branch};

fn extract_branch_name(options: &[String]) -> Option<&String> {
    options.iter().find(|&o| !o.starts_with("--"))
}

fn extract_create_option(options: &[String]) -> Option<&String> {
    options.iter().find(|&o| o == "--create")
}

pub fn handler(options: &[String]) -> Result<(), String> {
    let branch_name = match extract_branch_name(options) {
        Some(name) => name,
        None => return Err("Missing branch name".to_string()),
    };
    let create_option = match extract_create_option(options) {
        Some(_) => true,
        None => false,
    };

    if create_option {
        match create_branch(&branch_name) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    match checkout_branch(branch_name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

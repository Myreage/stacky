use crate::stacks::stacks::{checkout_stack, create_stack, print_current_stack};

fn extract_stack_name(options: &[String]) -> Option<&String> {
    options.iter().find(|&o| !o.starts_with("--"))
}

fn extract_create_option(options: &[String]) -> Option<&String> {
    options.iter().find(|&o| o == "--create")
}

pub fn handler(options: &[String]) -> Result<(), &'static str> {
    let stack_name_result = extract_stack_name(options);

    if stack_name_result.is_none() {
        match print_current_stack() {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }

    let stack_name = stack_name_result.unwrap();

    let create_option = match extract_create_option(options) {
        Some(_) => true,
        None => false,
    };

    if create_option {
        match create_stack(stack_name) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    match checkout_stack(stack_name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

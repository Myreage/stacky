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

mod handle_branch_command;
mod handle_stack_command;
mod persistence;
mod types;

/*
    stack <stack> => checkout stack
    stack --new <stack> => create and checkout stack
*/
pub fn handle_stack_command(options: &[String]) -> Result<(), &'static str> {
    let stack_name = match handle_stack_command::extract_stack_name(options) {
        Some(name) => name,
        None => return Err("Missing stack name"),
    };
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

/*
   branch <branch> => checkout branch belonging to current stack
   branch --new <branch> => create and checkout branch belonging to current stack

*/
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

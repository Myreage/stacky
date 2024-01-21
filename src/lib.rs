mod handle_stack_command;
mod persistence;
mod types;

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

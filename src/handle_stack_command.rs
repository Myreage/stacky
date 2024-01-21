use crate::{persistence, types};

pub fn create_stack(stack_name: &String) -> Result<(), &'static str> {
    let mut file_data = match persistence::read_from_file::<types::FileData>("save.json") {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file"),
    };

    if let Some(_) = file_data.stacks.iter().find(|&s| &s.name == stack_name) {
        return Err("Stack already exists");
    }

    file_data.stacks.push(types::Stack::new(stack_name));

    match persistence::write_to_file(&file_data, "save.json") {
        Ok(_) => {
            eprintln!("Created new stack {}", stack_name);
            Ok(())
        }
        Err(_) => Err("Failed writing to file"),
    }
}

pub fn checkout_stack(stack_name: &String) -> Result<(), &'static str> {
    let mut file_data = match persistence::read_from_file::<types::FileData>("save.json") {
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

pub fn extract_stack_name(options: &[String]) -> Option<&String> {
    options.iter().find(|&o| !o.starts_with("--"))
}

pub fn extract_create_option(options: &[String]) -> Option<&String> {
    options.iter().find(|&o| o == "--create")
}

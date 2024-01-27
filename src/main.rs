use std::env;

mod command_handlers;
mod git;
mod persistence;
mod stacks;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: stacky <command> [options]");
        return;
    }

    let command = &args[1];
    let options = &args[2..];

    let execution_result = match command.as_str() {
        "stack" => command_handlers::handle_stack_command::handler(options),
        "branch" => command_handlers::handle_branch_command::handler(options),
        "sync" => command_handlers::handle_sync_command::handler(),
        _ => Err("Unknown command".to_string()),
    };

    match execution_result {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

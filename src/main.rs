use std::env;

use command_handlers::{
    handle_branch_command::handler as handle_branch_command,
    handle_stack_command::handler as handle_stack_command,
    handle_sync_command::handler as handle_sync_command,
};

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
        "stack" => handle_stack_command(options),
        "branch" => handle_branch_command(options),
        "sync" => handle_sync_command(),
        _ => Err("Unknown command".to_string()),
    };

    match execution_result {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

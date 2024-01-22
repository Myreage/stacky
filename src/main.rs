use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: stacky <command> [options]");
        return;
    }

    let command = &args[1];
    let options = &args[2..];

    let execution_result = match command.as_str() {
        "stack" => stacky::handle_stack_command(options),
        "branch" => stacky::handle_branch_command(options),
        "sync" => stacky::handle_sync_command(),
        _ => Err("Unknown command"),
    };

    match execution_result {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

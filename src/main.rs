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
        "stack" => {
            /*
                stack <stack> => checkout stack
                stack --new <stack> => create and checkout stack
            */

            stacky::handle_stack_command(options)
        }
        "branch" => {
            /*
               branch <branch> => checkout branch belonging to current stack
               branch --new <branch> => create and checkout branch belonging to current stack

            */
            stacky::handle_branch_command(options)
        }
        "sync" => {
            /*
                sync => push/pull stack branches and updates/creates PRs of each stacked branch
            */
            Ok(())
        }
        _ => Err("Unknown command"),
    };

    match execution_result {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

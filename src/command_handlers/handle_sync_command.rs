use crate::{git, persistence};

pub fn handler() -> Result<(), String> {
    let branches = persistence::read_current_stack_branches()?;

    git::checkout_branch(&"main".to_string())?;
    git::pull_current_branch()?;

    for (index, branch) in branches.iter().enumerate() {
        git::checkout_branch(&branch.name)?;

        let branch_exists = git::check_branch_exists(&branch.name);

        if branch_exists {
            git::pull_current_branch()?;
        }

        let rebase_branch = match index {
            0 => "main",
            _ => &branches[index - 1].name,
        };

        git::rebase_current_branch(&rebase_branch.to_string())?;

        git::force_push_branch(&branch.name)?;

        git::open_pull_request(&rebase_branch.to_string(), &branch.name)
    }

    Ok(())
}

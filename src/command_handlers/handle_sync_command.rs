use crate::{
    git::{
        check_branch_exists, checkout_branch, force_push_branch, open_pull_request,
        pull_current_branch, rebase_current_branch,
    },
    persistence::read_current_stack_branches,
};

pub fn handler() -> Result<(), String> {
    let branches = read_current_stack_branches()?;

    checkout_branch(&"main".to_string())?;
    pull_current_branch()?;

    for (index, branch) in branches.iter().enumerate() {
        checkout_branch(&branch.name)?;

        let branch_exists = check_branch_exists(&branch.name);

        if branch_exists {
            pull_current_branch()?;
        }

        let rebase_branch = match index {
            0 => "main",
            _ => &branches[index - 1].name,
        };

        rebase_current_branch(&rebase_branch.to_string())?;

        force_push_branch(&branch.name)?;

        open_pull_request(&rebase_branch.to_string(), &branch.name)
    }

    Ok(())
}

use crate::{
    git,
    persistence::{self},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Branch {
    pub name: String,
}

impl Branch {
    pub fn new(name: &String) -> Branch {
        Branch {
            name: name.to_string(),
        }
    }
}

pub fn create_branch(branch_name: &String) -> Result<(), String> {
    git::create_branch(branch_name)?;
    persistence::save_branch_on_current_stack(branch_name)
}

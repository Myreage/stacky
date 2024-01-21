use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Stack {
    pub name: String,
    pub branches: Vec<Branch>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FileData {
    pub stacks: Vec<Stack>,
    pub current_stack: String,
}

impl Stack {
    pub fn new(name: &String) -> Stack {
        Stack {
            branches: vec![],
            name: name.to_string(),
        }
    }
}

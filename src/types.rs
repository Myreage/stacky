use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Branch {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stack {
    pub name: String,
    branches: Vec<Branch>,
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
            name: name.clone(),
        }
    }
}

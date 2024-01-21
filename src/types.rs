use std::fmt;

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

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Currently on stack {}\n\n", self.name)?;

        let mut line_size = 0;
        for (index, branch) in self.branches.iter().enumerate() {
            if index == 0 {
                write!(f, "{}\n", branch.name)?;
                line_size += branch.name.len();
            } else {
                let previous_element_size = self.branches.get(index - 1).unwrap().name.len();
                let number_of_spaces = line_size - previous_element_size / 2;

                for _i in 0..number_of_spaces {
                    write!(f, " ")?;
                }

                write!(f, "└──{}\n", branch.name)?;
                line_size = number_of_spaces + branch.name.len() + 3;
            }
        }

        Ok(())
    }
}

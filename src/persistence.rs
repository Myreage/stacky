use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::stacks::{branches::Branch, stacks::Stack};

#[derive(Debug, Serialize, Deserialize, Default)]
struct FileData {
    pub stacks: Vec<Stack>,
    pub current_stack: String,
}

fn write_to_file<T: Serialize>(data: &T, file_path: &PathBuf) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = io::BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, data)?;
    writer.flush()?;
    Ok(())
}

fn read_from_file<T: DeserializeOwned + Default + Serialize>(file_path: &PathBuf) -> io::Result<T> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            let _ = write_to_file(&T::default(), file_path);
            File::open(file_path)?
        }
    };

    let reader = io::BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

pub fn read_current_stack_branches() -> Result<Vec<Branch>, String> {
    let save_file_path = get_save_file_path()?;
    let file_data = match read_from_file::<FileData>(&save_file_path) {
        Ok(loaded_stacks) => loaded_stacks,
        Err(e) => return Err(format!("Error reading file {}", e)),
    };

    let current_stack = &file_data.current_stack;

    let branches = &file_data
        .stacks
        .iter()
        .find(|&s| &s.name == current_stack)
        .unwrap()
        .branches;

    Ok(branches.clone())
}

pub fn save_stack(stack_name: &String) -> Result<(), String> {
    let save_file_path = get_save_file_path()?;
    let mut file_data = match read_from_file::<FileData>(&save_file_path) {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file".to_string()),
    };

    if let Some(_) = file_data.stacks.iter().find(|&s| &s.name == stack_name) {
        return Err("Stack already exists".to_string());
    }

    file_data.stacks.push(Stack::new(stack_name));

    match write_to_file(&file_data, &save_file_path) {
        Ok(_) => {
            eprintln!("Created new stack {}", stack_name);
            Ok(())
        }
        Err(_) => Err("Failed writing to file".to_string()),
    }
}

pub fn save_branch_on_current_stack(branch_name: &String) -> Result<(), String> {
    let save_file_path = get_save_file_path()?;
    let mut file_data = match read_from_file::<FileData>(&save_file_path) {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file".to_string()),
    };

    let current_stack_position = match file_data
        .stacks
        .iter()
        .position(|s| s.name == file_data.current_stack)
    {
        Some(position) => position,
        None => return Err("Not currently on a stack".to_string()),
    };

    let current_stack = &mut file_data.stacks[current_stack_position];

    if let Some(_) = current_stack
        .branches
        .iter()
        .find(|&s| &s.name == branch_name)
    {
        return Err("Branch already exists".to_string());
    }

    current_stack.branches.push(Branch::new(branch_name));

    match write_to_file(&file_data, &save_file_path) {
        Ok(_) => {
            eprintln!("Created new branch {}", branch_name);
            Ok(())
        }
        Err(_) => Err("Failed writing to file".to_string()),
    }
}

pub fn save_current_stack(stack_name: &String) -> Result<(), String> {
    let save_file_path = get_save_file_path()?;
    let mut file_data = match read_from_file::<FileData>(&save_file_path) {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file".to_string()),
    };

    match file_data.stacks.iter().find(|&s| &s.name == stack_name) {
        Some(_) => (),
        None => return Err("Stack not found".to_string()),
    };

    file_data.current_stack = stack_name.clone();

    match write_to_file(&file_data, &save_file_path) {
        Ok(_) => {
            eprintln!("On stack {}", stack_name);
            Ok(())
        }
        Err(_) => Err("Failed writing to file".to_string()),
    }
}

pub fn read_current_stack() -> Result<Stack, String> {
    let save_file_path = get_save_file_path()?;
    dbg!(&save_file_path);
    let file_data = match read_from_file::<FileData>(&save_file_path) {
        Ok(loaded_stacks) => loaded_stacks,
        Err(e) => return Err(e.to_string()),
    };

    let current_stack = match file_data
        .stacks
        .iter()
        .find(|&s| s.name == file_data.current_stack)
    {
        Some(stack) => stack,
        None => return Err("Not currently on a stack".to_string()),
    };

    Ok(current_stack.clone())
}

fn find_git_parent_dir() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().ok()?;

    let git_dir = current_dir.join(".git");

    if git_dir.is_dir() {
        return Some(current_dir);
    }

    while current_dir.pop() {
        let git_dir = current_dir.join(".git");

        if git_dir.is_dir() {
            return Some(current_dir);
        }
    }

    None
}

fn create_stacky_folder_and_save_json(parent_dir: &Path) -> Result<PathBuf, String> {
    let stacky_dir = parent_dir.join(".stacky");
    let save_json_path = stacky_dir.join("save.json");

    if !stacky_dir.is_dir() {
        fs::create_dir_all(&stacky_dir).map_err(|e| e.to_string())?;
    }

    Ok(save_json_path)
}

fn get_save_file_path() -> Result<PathBuf, String> {
    if let Some(git_parent_dir) = find_git_parent_dir() {
        create_stacky_folder_and_save_json(&git_parent_dir)
    } else {
        Err("Aucun dossier '.git' trouvé dans l'arborescence.".to_string())
    }
}

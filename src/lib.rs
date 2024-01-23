use std::{
    io::{self, Write},
    process::Command,
    str,
};

use reqwest::blocking::{Client, RequestBuilder};
use serde_json::json;

mod handle_branch_command;
mod handle_stack_command;
mod persistence;
mod types;

pub fn handle_stack_command(options: &[String]) -> Result<(), &'static str> {
    let stack_name_result = handle_stack_command::extract_stack_name(options);

    if stack_name_result.is_none() {
        match handle_stack_command::print_current_stack() {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }

    let stack_name = stack_name_result.unwrap();

    let create_option = match handle_stack_command::extract_create_option(options) {
        Some(_) => true,
        None => false,
    };

    if create_option {
        match handle_stack_command::create_stack(stack_name) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    match handle_stack_command::checkout_stack(stack_name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn handle_branch_command(options: &[String]) -> Result<(), &'static str> {
    let branch_name = match handle_branch_command::extract_branch_name(options) {
        Some(name) => name,
        None => return Err("Missing branch name"),
    };
    let create_option = match handle_branch_command::extract_create_option(options) {
        Some(_) => true,
        None => false,
    };

    if create_option {
        match handle_branch_command::create_branch(&branch_name) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    match handle_branch_command::checkout_branch(branch_name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn handle_sync_command() -> Result<(), &'static str> {
    let file_data = match persistence::read_from_file::<types::FileData>("save.json") {
        Ok(loaded_stacks) => loaded_stacks,
        Err(_) => return Err("Error when reading file"),
    };

    let current_stack = file_data.current_stack;

    let branches = &file_data
        .stacks
        .iter()
        .find(|&s| s.name == current_stack)
        .unwrap()
        .branches;

    // Pull main

    let mut git_checkout_main = Command::new("git");
    git_checkout_main.arg("checkout").arg("main");

    match git_checkout_main.output() {
        Ok(result) => {
            if !result.status.success() {
                io::stdout().write_all(&result.stdout).unwrap();
                io::stderr().write_all(&result.stderr).unwrap();
                return Err("Git checkout failed");
            }
        }
        Err(_) => return Err("Git checkout failed"),
    }

    let mut git_pull_main = Command::new("git");
    git_pull_main.arg("pull");

    match git_pull_main.output() {
        Ok(result) => {
            if !result.status.success() {
                io::stdout().write_all(&result.stdout).unwrap();
                io::stderr().write_all(&result.stderr).unwrap();
                return Err("Git pull main failed");
            }
        }
        Err(_) => return Err("Git pull main failed"),
    }

    // Rebase and push all branches
    for (index, branch) in branches.iter().enumerate() {
        let mut git_checkout = Command::new("git");
        git_checkout.arg("checkout").arg(&branch.name);

        match git_checkout.output() {
            Ok(result) => {
                if !result.status.success() {
                    io::stdout().write_all(&result.stdout).unwrap();
                    io::stderr().write_all(&result.stderr).unwrap();
                    return Err("Git checkout failed");
                }
            }
            Err(_) => return Err("Git checkout failed"),
        }

        let ls_remote_command = Command::new("git")
            .args(&["ls-remote", "--exit-code", "origin", &branch.name])
            .output();

        let branch_exists = match ls_remote_command {
            Ok(output) => output.status.success(),
            Err(_) => false,
        };

        if branch_exists {
            let mut git_pull = Command::new("git");
            git_pull.arg("pull");

            match git_pull.output() {
                Ok(result) => {
                    if !result.status.success() {
                        dbg!(&result);
                        io::stdout().write_all(&result.stdout).unwrap();
                        io::stderr().write_all(&result.stderr).unwrap();
                        return Err("Git pull failed");
                    }
                }
                Err(_) => return Err("Git checkout failed"),
            }
        }

        let rebase_branch = match index {
            0 => "main",
            _ => &branches[index - 1].name,
        };

        let mut git_rebase = Command::new("git");
        git_rebase.arg("rebase").arg(rebase_branch);

        match git_rebase.output() {
            Ok(result) => {
                if !result.status.success() {
                    io::stdout().write_all(&result.stdout).unwrap();
                    io::stderr().write_all(&result.stderr).unwrap();
                    return Err("Git rebase failed");
                }
            }
            Err(_) => return Err("Git rebase failed"),
        }

        let mut git_push = Command::new("git");
        git_push
            .arg("push")
            .arg("--force")
            .arg("--set-upstream")
            .arg("origin")
            .arg(&branch.name);

        match git_push.output() {
            Ok(result) => {
                if !result.status.success() {
                    io::stdout().write_all(&result.stdout).unwrap();
                    io::stderr().write_all(&result.stderr).unwrap();
                    return Err("Git push failed");
                }
            }
            Err(_) => return Err("Git push failed"),
        }

        // PULL REQUEST OPENING
        // Extrait le propriétaire et le nom du dépôt à partir du dépôt Git
        let (repo_owner, repo_name) = extract_repo_owner_and_name()
            .expect("Impossible d'extraire les informations du dépôt Git.");

        // Remplacez ces valeurs par les informations de votre repository et votre token d'accès personnel
        let base_branch = rebase_branch;
        let head_branch = branch.name.as_str();
        let access_token = "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";

        // Construire l'URL de l'API GitHub pour créer une pull request
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/pulls",
            repo_owner, repo_name
        );

        // Construire la requête HTTP
        let request_builder =
            create_pull_request_request(api_url, access_token, base_branch, head_branch);

        // Exécuter la requête
        match request_builder.send() {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Pull request créée avec succès!");
                } else {
                    println!(
                        "Erreur lors de la création de la pull request: {:?}",
                        response
                    );
                }
            }
            Err(e) => {
                println!("Erreur lors de la requête HTTP: {:?}", e);
            }
        }
    }

    Ok(())
}

fn create_pull_request_request(
    api_url: String,
    access_token: &str,
    base: &str,
    head: &str,
) -> RequestBuilder {
    // Construire la requête HTTP POST pour créer une pull request
    let client = Client::new();
    let request_builder = client
        .post(&api_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&json!({
            "title": "Titre de la pull request",
            "body": "Description de la pull request",
            "base": base,
            "head": head,
        }));

    request_builder
}

fn extract_repo_owner_and_name() -> Option<(String, String)> {
    // Exécute la commande `git remote -v` et capture la sortie
    let output = Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .expect("La commande git a échoué");

    dbg!(&output);

    // Convertit la sortie du processus en une chaîne de caractères
    let output_str = str::from_utf8(&output.stdout).ok()?;

    dbg!(output_str);

    // Extrait le propriétaire du dépôt et le nom du dépôt à partir de la sortie
    extract_owner_and_name_from_url(output_str.to_string())
}

fn extract_owner_and_name_from_url(url: String) -> Option<(String, String)> {
    // Supprime le préfixe "git@" si présent
    let cleaned_url = if url.starts_with("git@") {
        &url[4..]
    } else {
        &url
    };

    dbg!(cleaned_url);

    // Supprime le suffixe ".git" si présent
    let cleaned_url = if cleaned_url.ends_with(".git") {
        &cleaned_url[..cleaned_url.len() - 4]
    } else {
        cleaned_url
    };

    dbg!(cleaned_url);

    // Divise l'URL en parties en utilisant le séparateur ":"
    let parts: Vec<&str> = cleaned_url.split(':').collect();

    dbg!(&parts);

    // Si l'URL est dans le format attendu, retourne le propriétaire et le nom du dépôt
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

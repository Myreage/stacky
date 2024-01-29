use std::{collections::HashMap, env, io, process::Command, str::from_utf8};

use reqwest::blocking::{Client, RequestBuilder};
use serde::Deserialize;

pub fn create_pull_request_request(
    api_url: String,
    access_token: &str,
    base: &str,
    head: &str,
    title: &str,
) -> RequestBuilder {
    // Construire la requête HTTP POST pour créer une pull request
    let client = Client::new();

    let mut body = HashMap::new();
    body.insert("title", title);
    body.insert("body", "Description de la Pull Request");
    body.insert("base", base);
    body.insert("head", head);

    let request_builder = client
        .post(&api_url)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "stacky")
        .json(&body);

    request_builder
}

pub fn checkout_branch(branch_name: &String) -> Result<(), String> {
    let mut git_checkout_main = Command::new("git");
    git_checkout_main.arg("checkout").arg(branch_name);

    match git_checkout_main.output() {
        Ok(result) => {
            if !result.status.success() {
                let stdout = from_utf8(&result.stdout).unwrap();
                let stderr = from_utf8(&result.stderr).unwrap();
                let formatted_error = format!("{}\n{}\nGit checkout failed", stdout, stderr);
                return Err(formatted_error);
            }
            Ok(())
        }
        Err(_) => return Err("Git checkout failed".to_string()),
    }
}

pub fn create_branch(branch_name: &String) -> Result<(), String> {
    let mut git_branch = Command::new("git");
    git_branch.arg("branch").arg(branch_name);

    match git_branch.output() {
        Ok(result) => {
            if !result.status.success() {
                let stdout = from_utf8(&result.stdout).unwrap();
                let stderr = from_utf8(&result.stderr).unwrap();
                let formatted_error = format!("{}\n{}\nGit branch failed", stdout, stderr);
                return Err(formatted_error);
            }
            Ok(())
        }
        Err(_) => return Err("Git branch failed".to_string()),
    }
}

pub fn pull_current_branch() -> Result<(), String> {
    let mut git_pull_main = Command::new("git");
    git_pull_main.arg("pull");

    match git_pull_main.output() {
        Ok(result) => {
            if !result.status.success() {
                let stdout = from_utf8(&result.stdout).unwrap();
                let stderr = from_utf8(&result.stderr).unwrap();
                let formatted_error = format!("{}\n{}\nGit pull failed", stdout, stderr);
                return Err(formatted_error);
            }
            Ok(())
        }
        Err(_) => return Err("Git pull main failed".to_string()),
    }
}

pub fn check_branch_exists(branch_name: &String) -> bool {
    let ls_remote_command = Command::new("git")
        .args(&["ls-remote", "--exit-code", "origin", &branch_name])
        .output();

    match ls_remote_command {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub fn find_open_pull_request_request(
    access_token: &str,
    base: &str,
    head: &str,
    repo_owner: &str,
    repo_name: &str,
) -> RequestBuilder {
    let client = Client::new();

    let api_url = format!(
        "https://api.github.com/repos/{}/{}/pulls",
        repo_owner, repo_name
    );

    let head_filter = format!("{}:{}", repo_owner, head);

    let request_builder = client
        .get(&api_url)
        .query(&[
            ("state", "open"),
            ("head", head_filter.as_str()),
            ("base", base),
        ])
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "stacky");

    request_builder
}

pub fn rebase_current_branch(target_branch_name: &String) -> Result<(), String> {
    let mut git_rebase = Command::new("git");
    git_rebase.arg("rebase").arg(target_branch_name);

    match git_rebase.output() {
        Ok(result) => {
            if !result.status.success() {
                let stdout = from_utf8(&result.stdout).unwrap();
                let stderr = from_utf8(&result.stderr).unwrap();
                let formatted_error = format!("{}\n{}\nGit rebase failed", stdout, stderr);
                return Err(formatted_error);
            }
            Ok(())
        }
        Err(_) => return Err("Git rebase failed".to_string()),
    }
}

pub fn force_push_branch(origin_branch_name: &String) -> Result<(), String> {
    let mut git_push = Command::new("git");
    git_push
        .arg("push")
        .arg("--force")
        .arg("--set-upstream")
        .arg("origin")
        .arg(origin_branch_name);

    match git_push.output() {
        Ok(result) => {
            if !result.status.success() {
                let stdout = from_utf8(&result.stdout).unwrap();
                let stderr = from_utf8(&result.stderr).unwrap();
                let formatted_error = format!("{}\n{}\nGit push failed", stdout, stderr);
                return Err(formatted_error);
            }
            Ok(())
        }
        Err(_) => return Err("Git push failed".to_string()),
    }
}

#[derive(Deserialize, Clone)]
pub struct FindPullRequestResponse {}

pub fn find_pull_request(
    base_branch_name: &String,
    head_branch_name: &String,
) -> Result<Option<FindPullRequestResponse>, String> {
    let (repo_owner, repo_name) = extract_repo_owner_and_name()
        .expect("Impossible d'extraire les informations du dépôt Git.");

    let access_token = env::var("API_KEY").unwrap();

    let request_builder = find_open_pull_request_request(
        &access_token,
        base_branch_name,
        head_branch_name,
        &repo_owner,
        &repo_name,
    );

    match request_builder.send() {
        Ok(response) => {
            if response.status().is_success() {
                let content: Vec<FindPullRequestResponse> = response.json().unwrap();
                let pull_request = content.get(0);

                Ok(pull_request.cloned())
            } else {
                Err(format!(
                    "Erreur lors de la récupération des pull requests: {:?}",
                    response.text()
                ))
            }
        }
        Err(e) => Err(format!(
            "Erreur lors de la récupération des pull requests: {:?}",
            e
        )),
    }
}

pub fn open_pull_request(base_branch_name: &String, head_branch_name: &String) {
    let (repo_owner, repo_name) = extract_repo_owner_and_name()
        .expect("Impossible d'extraire les informations du dépôt Git.");

    let mut pr_title = String::new();

    println!(
        "Opening a new Pull Request for {} <- {}",
        base_branch_name, head_branch_name
    );
    println!("Enter a Pull Request title");

    io::stdin()
        .read_line(&mut pr_title)
        .expect("Erreur lors de la lecture de l'entrée");

    let access_token = env::var("API_KEY").unwrap();

    let api_url = format!(
        "https://api.github.com/repos/{}/{}/pulls",
        repo_owner, repo_name
    );

    // Construire la requête HTTP
    let request_builder = create_pull_request_request(
        api_url,
        &access_token,
        &base_branch_name,
        &head_branch_name,
        &pr_title,
    );

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

fn extract_repo_owner_and_name() -> Option<(String, String)> {
    let output = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .expect("La commande git a échoué");

    let output_str = std::str::from_utf8(&output.stdout).ok()?;

    let stripped_output = output_str.strip_prefix("https://github.com/")?;

    let parts: Vec<&str> = stripped_output.split("/").collect();

    if parts.len() >= 2 {
        let owner_name = parts[0];
        let repo_name = parts[1].trim_end_matches(".git\n");

        Some((owner_name.to_string(), repo_name.to_string()))
    } else {
        None
    }
}

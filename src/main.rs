use regex::Regex;
use std::env;
use std::fs;
use std::io::{self};
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process::{exit, Command, Output};

fn run_command(cmd: &str, args: &[&str]) -> Result<Output, io::Error> {
    let output = Command::new(cmd).args(args).output(); // 実行結果を取得

    match output {
        Ok(output) => {
            if !output.stderr.is_empty() {
                eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
            }
            Ok(output)
        }
        Err(e) => {
            eprintln!("Failed to execute: {} {:?}", cmd, args);
            Err(e)
        }
    }
}

fn run_login() {
    let status = Command::new("gcloud").arg("auth").arg("login").status();

    match status {
        Ok(status) if status.success() => {
            println!("gcloud auth login executed successfully.");
        }
        Ok(status) => {
            println!("gcloud auth login failed with status: {}", status);
            exit(1);
        }
        Err(e) => {
            println!("Failed to execute gcloud auth login: {}", e);
            exit(1);
        }
    }
}

fn main() {
    let profile_name =
        env::var("MY_GCP_PROFILE_NAME").unwrap_or_else(|_| "default_profile".to_string());
    let account_name =
        env::var("MY_GCP_ACCOUNT_NAME").unwrap_or_else(|_| "default_account".to_string());
    let project_id =
        env::var("MY_GCP_PROJECT_ID").unwrap_or_else(|_| "default_project".to_string());
    let region = env::var("MY_GCP_REGION").unwrap_or_else(|_| "default_region".to_string());
    let zone = env::var("MY_GCP_ZONE").unwrap_or_else(|_| "default_zone".to_string());

    let credentials_file = format!(
        "{}/.config/gcloud/application_default_credentials.json",
        env::var("HOME").unwrap()
    );
    let credentials_dir = format!(
        "{}/.config/gcloud/tmp_credentials",
        env::var("HOME").unwrap()
    );
    let tmp_credentials_file = format!("{}/{}.json", credentials_dir, profile_name);

    println!("Enter Project: {}", project_id);

    check_wrap(
        &profile_name,
        &account_name,
        &project_id,
        &region,
        &zone,
        &credentials_file,
        &credentials_dir,
        &tmp_credentials_file,
    );

    println!("\nDone.");
}

fn check_wrap(
    profile_name: &str,
    account_name: &str,
    project_id: &str,
    region: &str,
    zone: &str,
    credentials_file: &str,
    credentials_dir: &str,
    tmp_credentials_file: &str,
) {
    println!("Check configure gcloud.");
    gcloud_login_check(account_name);

    if !Path::new(credentials_dir).exists() {
        fs::create_dir_all(credentials_dir).expect("Failed to create credentials directory");
    }

    if Path::new(credentials_file).exists() {
        fs::remove_file(credentials_file).expect("Failed to remove credentials file");
    }

    gcloud_credentials_set_check(tmp_credentials_file, credentials_file);
    gcloud_config_set_check(profile_name, account_name, project_id, region, zone);
    gcloud_config_active_check(profile_name);

    symlink(tmp_credentials_file, credentials_file).expect("Failed to create symlink");
    gcloud_application_login_check();
}

fn gcloud_login_check(account_name: &str) {
    let output = run_command(
        "gcloud",
        &["auth", "list", "--format=value(account, status)"],
    );
    let output = output.unwrap();
    let accounts = String::from_utf8_lossy(&output.stdout);

    // 正規表現で "*" を前に持つアカウントを検出
    let pattern = format!(r"{}(\s+\*)", account_name);
    let re = Regex::new(&pattern).unwrap();

    if !re.is_match(&accounts) {
        println!(" Account: {} is not active.", account_name);
        println!();
        println!("Please login to gcloud.");
        run_login();
        println!();
    } else {
        println!(" Account {} is active.", account_name);
    }
}

fn gcloud_credentials_set_check(tmp_credentials_file: &str, credentials_file: &str) {
    if Path::new(tmp_credentials_file).exists() {
        println!(" File {} exists.", tmp_credentials_file);
    } else {
        println!(" File {} does not exist.", tmp_credentials_file);
        gcloud_application_login();
        fs::copy(credentials_file, tmp_credentials_file).expect("Failed to copy credentials file");
    }
}

fn gcloud_application_login_check() {
    let output = run_command(
        "gcloud",
        &["auth", "application-default", "print-access-token"],
    );

    let output = output.unwrap();
    if !output.status.success() {
        println!(
            " Application login failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        gcloud_application_login();
    } else {
        println!(" Application login is ok.");
    }
}

fn gcloud_application_login() {
    println!();
    println!("Please application login to gcloud.");
    if let Err(e) = run_command("gcloud", &["auth", "application-default", "login"]) {
        eprintln!("Failed to login: {}", e);
    }
    println!();
}

fn gcloud_config_set_check(
    profile_name: &str,
    account_name: &str,
    project_id: &str,
    region: &str,
    zone: &str,
) {
    let output = run_command("gcloud", &["config", "configurations", "list"]);
    let output = output.unwrap();
    let config_list = String::from_utf8_lossy(&output.stdout);

    if !config_list.contains(profile_name) {
        println!(" Profile {} not found.", profile_name);
        gcloud_config_set(profile_name, account_name, project_id, region, zone);
    } else {
        println!(" Profile {} found.", profile_name);
    }
}

fn gcloud_config_active_check(profile_name: &str) {
    let output = run_command(
        "gcloud",
        &["config", "configurations", "list", "--filter=active=True"],
    );
    let output = output.unwrap();
    let active_profile = String::from_utf8_lossy(&output.stdout);

    if !active_profile.contains(profile_name) {
        println!(" Profile {} is not active.", profile_name);
        if let Err(e) = run_command(
            "gcloud",
            &["config", "configurations", "activate", profile_name],
        ) {
            eprintln!("Failed to activate profile {}: {}", profile_name, e);
        }
    } else {
        println!(" Profile {} is active.", profile_name);
    }
}

fn gcloud_config_set(
    profile_name: &str,
    account_name: &str,
    project_id: &str,
    region: &str,
    zone: &str,
) {
    let commands = [
        ("configurations", "create", profile_name),
        ("set", "account", account_name),
        ("set", "project", project_id),
        ("set", "compute/region", region),
        ("set", "compute/zone", zone),
    ];

    for &(cmd, arg, value) in &commands {
        if let Err(e) = run_command("gcloud", &["config", cmd, arg, value]) {
            eprintln!("Failed to {} {}: {}", cmd, arg, e);
        }
    }
}

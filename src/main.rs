use std::env;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process::{Command, Output};

fn run_command(cmd: &str, args: &[&str]) -> Output {
    Command::new(cmd)
        .args(args)
        .output()
        .expect(&format!("Failed to execute: {} {:?}", cmd, args))
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
    let accounts = String::from_utf8_lossy(&output.stdout);

    if !accounts.contains(&format!("{} *", account_name)) {
        println!(" Account: {} is not active.", account_name);
        println!();
        println!("Please login to gcloud.");
        run_command("gcloud", &["auth", "login"]);
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
    run_command("gcloud", &["auth", "application-default", "login"]);
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
    let active_profile = String::from_utf8_lossy(&output.stdout);

    if !active_profile.contains(profile_name) {
        println!(" Profile {} is not active.", profile_name);
        run_command(
            "gcloud",
            &["config", "configurations", "activate", profile_name],
        );
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
    run_command(
        "gcloud",
        &["config", "configurations", "create", profile_name],
    );
    run_command("gcloud", &["config", "set", "account", account_name]);
    run_command("gcloud", &["config", "set", "project", project_id]);
    run_command("gcloud", &["config", "set", "compute/region", region]);
    run_command("gcloud", &["config", "set", "compute/zone", zone]);
}

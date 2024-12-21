use std::process::{exit, Command};

fn main() {
    let status = Command::new("gcloud")
        .arg("auth")
        .arg("application-default")
        .arg("login")
        .status();

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

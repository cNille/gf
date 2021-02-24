use std::process::Command;

pub fn git_command(args: Vec<String>) -> String {
    let mut git_cmd = Command::new("git");

    for arg in args {
        git_cmd.arg(arg);
    }
    let output = git_cmd
        .output()
        .expect("git command failed. Ensure it is installed.");

    let result = String::from_utf8_lossy(&output.stdout).to_string();
    // println!("{}", result);
    return result;
}

pub fn pwd_command() -> String {
    let pwd_output = Command::new("pwd")
        .output()
        .expect("Unable to use `pwd` to check working-directory.");
    let repo_str = String::from_utf8_lossy(&pwd_output.stdout).to_string();
    let repo = repo_str.trim();
    return repo.to_string();
}

pub fn git_status() -> String {
    let status = Command::new("git")
        .arg("status")
        .arg("-s")
        .output()
        .expect("git command failed. Ensure it is installed.");
    return String::from_utf8_lossy(&status.stdout).to_string();
}

pub fn git_diff(file_name: String) -> String {
    let status = Command::new("git")
        .arg("diff")
        .arg(file_name)
        .output()
        .expect("git command failed. Ensure it is installed.");

    let result = String::from_utf8_lossy(&status.stdout).to_string();
    return result ;
}


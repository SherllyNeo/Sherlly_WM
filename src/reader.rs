
use regex::Regex;
use std::process::Command;
pub fn reader(file_path: &str) -> Option<String> {
    let command = "sh".to_owned();
    let output = Command::new(command).arg(file_path).output().expect("failed to run shell");
    let text = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new("\n").unwrap();
    let cleaned_text = re.replace(&text,"");
    Some(format!("{:?}",cleaned_text))
}

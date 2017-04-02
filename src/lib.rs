use std::process::Command;

pub fn get_content(url : &str) -> Option<String> {
    let output = Command::new("curl")
        .arg(url)
        .output()
        .expect("Process failed! Check your curl dependencies");

    let result = String::from_utf8_lossy(&output.stdout).into_owned();
    
    if output.status.success() {
       return Some(result);
    }
    
     None
}

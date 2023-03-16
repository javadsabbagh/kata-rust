use std::process::Command;

fn main() {
    let output = Command::new("ps")
        .arg("aux")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("ps aux succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("ps aux failed and stderr was:\n{}", s);
    }
}

use std::process::Command;

fn main() {
    let _editor_status = Command::new("nvim").arg("./temp.txt").status();
}

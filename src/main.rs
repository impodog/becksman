#[cfg(debug_assertions)]
fn start_server() -> std::process::Child {
    let child = std::process::Command::new("cargo")
        .args(["run", "-p", "becks_server"])
        .stdin(std::process::Stdio::null())
        .spawn()
        .unwrap();
    child
}

fn main() {
    let mut server = start_server();
    server.wait().unwrap();
}

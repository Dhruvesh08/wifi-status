use std::process::Command;

fn main() {
    let output = Command::new("ifconfig")
        .output()
        .expect("Failed to execute ifconfig command");

    let stdout = String::from_utf8(output.stdout).expect("Failed to convert stdout to string");

    if stdout.contains("wlan0") {
        println!("WiFi is on.");
    } else {
        println!("WiFi is off.");
    }
}
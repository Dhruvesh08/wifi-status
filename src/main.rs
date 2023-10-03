use std::process::Command;

fn is_wifi_on() -> bool {
    let output = Command::new("ifconfig")
        .output()
        .expect("Failed to execute ifconfig command");

    let stdout = String::from_utf8(output.stdout).expect("Failed to convert stdout to string");

    stdout.contains("wlan0")
}

fn main() {
    let is_wifi_on = is_wifi_on();

    if is_wifi_on {
        println!("WiFi is on.");
    } else {
        println!("WiFi is off.");
    }
}
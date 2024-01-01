use std::env;
use totp_rs::{Algorithm, TOTP, Secret};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let secret = &args[1];
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(secret.to_string()).to_bytes().unwrap(),
    ).unwrap();
    let token = totp.generate_current().unwrap();
    let url = format!("http://127.0.0.1:4646/ffxivlauncher/{}", token);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("body = {:?}", body);

    Ok(())
}
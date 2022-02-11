use reqwest::blocking;
use std::error::Error;

pub fn get(url: &str) -> Result<bool, Box<dyn Error>> {
    let builder = blocking::Client::builder();
    let client = builder.build()?;
    let response = client.get(url).send()?;
    let status = response.status();
    Ok(status.is_success())
}

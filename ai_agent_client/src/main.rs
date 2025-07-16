use dialoguer::Input;
use reqwest::blocking::Client;
use serde_json::json;

fn main() {
    let client = Client::new();
    loop {
        let input: String = Input::new().with_prompt("You").interact_text().unwrap();
        if input.trim() == "exit" { break; }

        let res = client.post("http://127.0.0.1:3000/command")
            .json(&json!({ "command": input }))
            .send();

        match res {
            Ok(resp) => {
                let text = resp.text().unwrap_or_else(|_| "No response".to_string());
                println!("Agent: {}", text);
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::process::Command;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;

/// Structs to deserialize OpenAI's API response
#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // Retrieve command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Box::from(format!("Usage: {} <commit_msg_file>", args[0])));
    }
    let commit_msg_file = &args[1];

    // Get the git diff of staged changes
    let diff_output = get_git_diff()?;

    // Prepare the prompt for OpenAI
    let prompt = format!(
        "Write a great commit message for the following diff. Only output the commit message, no other text. Follow a note about git commit messages by Tim Pope. Use a blank line between paragraphs. Make sure the title line is 50 characters or less.\n\n{}",
        diff_output
    );

    // Get the commit message from OpenAI
    let commit_message = format_commit_message_max_72_chars(prompt_openai(&prompt)?);

    // Read the existing commit message
    let current_message = match fs::read_to_string(commit_msg_file) {
        Ok(content) => content,
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(Box::new(e)),
    };

    // Combine the new commit message with the existing one
    let new_message = if current_message.is_empty() {
        commit_message
    } else {
        format!("{}\n{}", commit_message, current_message)
    };

    // Write the combined commit message back to the file
    fs::write(commit_msg_file, new_message)?;

    Ok(())
}

/// Executes `git diff --cached` and returns the output as a String
fn get_git_diff() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(&[
            "diff",
            "--cached",
            "--",
            ":!package-lock.json",
            ":!Cargo.lock",
            ":!yarn.lock",
            ":!pnpm-lock.yaml",
            ":!bun.lockb",
            ":!pnpm-lock.yaml",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Box::from(format!("Error running git diff: {}", stderr)));
    }

    let diff = String::from_utf8(output.stdout)?;
    Ok(diff)
}

/// Sends the prompt to OpenAI's API and returns the generated commit message
fn prompt_openai(prompt: &str) -> Result<String, Box<dyn Error>> {
    let api_key =
        env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY environment variable is not set")?;

    let client = Client::new();

    let request_body = serde_json::json!({
        "model": "gpt-4o-mini",
        "messages": [
            { "role": "user", "content": prompt }
        ],
        "temperature": 0.7,
        "max_tokens": 500
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .header(CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().unwrap_or_default();
        return Err(Box::from(format!(
            "OpenAI API error {}: {}",
            status, error_text
        )));
    }

    let response_data: OpenAIResponse = response.json()?;

    if let Some(choice) = response_data.choices.into_iter().next() {
        Ok(choice.message.content.trim().to_string())
    } else {
        Err(Box::from("No choices found in OpenAI response"))
    }
}

/// Formats the commit message to ensure each line does not exceed 72 characters
fn format_commit_message_max_72_chars(commit_message: String) -> String {
    let mut formatted = String::new();

    for paragraph in commit_message.split("\n\n") {
        let mut current_line = String::new();

        for word in paragraph.split_whitespace() {
            if current_line.len() + word.len() + 1 > 72 {
                if !current_line.is_empty() {
                    formatted.push_str(&current_line.trim_end());
                    formatted.push('\n');
                    current_line.clear();
                }
            }

            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }

        if !current_line.is_empty() {
            formatted.push_str(&current_line.trim_end());
            formatted.push('\n');
        }

        formatted.push('\n'); // Add a blank line between paragraphs
    }

    // Remove the last extra newline
    if formatted.ends_with("\n\n") {
        formatted.truncate(formatted.len() - 1);
    }

    formatted.trim_end().to_string()
}

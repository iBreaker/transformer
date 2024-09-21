#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use std::fs;
use std::path::PathBuf;
use log::{info, error};
use env_logger::Env;
use std::fmt;

#[derive(Serialize, Deserialize)]
struct TranslationRequest {
    text: String,
    source_lang: String,
    target_lang: String,
}

impl fmt::Debug for TranslationRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TranslationRequest")
            .field("source_lang", &self.source_lang)
            .field("target_lang", &self.target_lang)
            .finish()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatCompletionMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatCompletionMessage>,
    stream: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    api_url: String,
    api_key: String,
    model: String,
}

fn get_config_path() -> PathBuf {
    let mut path = tauri::api::path::config_dir().expect("Failed to get config directory");
    path.push("translator_config.json");
    path
}

fn load_config() -> Config {
    let path = get_config_path();
    if path.exists() {
        let contents = fs::read_to_string(path).expect("Failed to read config file");
        match serde_json::from_str(&contents) {
            Ok(config) => config,
            Err(_) => {
                // If parsing fails, return default config
                info!("Failed to parse existing config, using default");
                Config {
                    api_url: String::from("https://api.openai.com/v1/chat/completions"),
                    api_key: String::new(),
                    model: String::from("gpt-3.5-turbo"),
                }
            }
        }
    } else {
        Config {
            api_url: String::from("https://api.openai.com/v1/chat/completions"),
            api_key: String::new(),
            model: String::from("gpt-3.5-turbo"),
        }
    }
}

fn save_config(config: &Config) -> Result<(), String> {
    let path = get_config_path();
    let contents = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(path, contents).map_err(|e| e.to_string())
}

#[tauri::command]
async fn translate(
    window: tauri::Window,
    request: TranslationRequest,
) -> Result<(), String> {
    info!("Translation request received: {:?}", request);
    let config = load_config();
    let client = Client::new();

    info!("Translating: {:?}", request);
    info!("Using API URL: {}", config.api_url);

    let chat_request = ChatCompletionRequest {
        model: config.model.clone(),
        messages: vec![
            ChatCompletionMessage {
                role: "system".to_string(),
                content: format!("You are a translator. Translate from {} to {}.", request.source_lang, request.target_lang),
            },
            ChatCompletionMessage {
                role: "user".to_string(),
                content: request.text,
            },
        ],
        stream: true,
    };

    // 添加调试输出
    info!("Request body: {:?}", chat_request);

    info!("Sending request to OpenAI API");
    let response = client
        .post(&config.api_url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&chat_request)
        .send()
        .await
        .map_err(|e| {
            let error_msg = format!("API request failed: {}", e);
            error!("{}", error_msg);
            window.emit("translation_error", &error_msg).unwrap();
            error_msg
        })?;

    if !response.status().is_success() {
        let error_msg = format!("API request failed with status: {}", response.status());
        error!("{}", error_msg);
        window.emit("translation_error", &error_msg).unwrap();
        return Err(error_msg);
    }

    info!("Received response from OpenAI API");

    let mut translation = String::new();

    let bytes = response.bytes().await.map_err(|e| {
        let error_msg = format!("Error reading response: {}", e);
        error!("{}", error_msg);
        window.emit("translation_error", &error_msg).unwrap();
        error_msg
    })?;

    let content = String::from_utf8_lossy(&bytes);
    for line in content.lines() {
        if line.starts_with("data: ") && line != "data: [DONE]" {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line[6..]) {
                if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                    translation.push_str(content);
                    info!("Translation progress: {}", translation);
                    window.emit("translation_progress", &translation).map_err(|e| e.to_string())?;
                }
            }
        }
    }

    info!("Translation completed");
    Ok(())
}

#[tauri::command]
fn get_config() -> Config {
    load_config()
}

#[tauri::command]
fn set_config(config: Config) -> Result<(), String> {
    save_config(&config)
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .format_timestamp(None)
        .init();

    info!("Starting application");

    // Migrate existing config
    let config = load_config();
    save_config(&config).expect("Failed to migrate config");

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                info!("DevTools opened");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![translate, get_config, set_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

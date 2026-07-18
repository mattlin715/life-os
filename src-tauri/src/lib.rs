mod sqlite;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AiRuntimeStatus {
    provider: String,
    model: Option<String>,
    reason: Option<String>,
}

#[tauri::command]
fn get_ai_runtime_status() -> AiRuntimeStatus {
    match preferred_ai_provider().as_deref() {
        Some("gemini") => gemini_runtime_status(),
        Some("openai") => openai_runtime_status(),
        Some(other) => AiRuntimeStatus {
            provider: "mock".to_string(),
            model: None,
            reason: Some(format!(
                "AI_PROVIDER is set to '{other}', but only 'gemini' or 'openai' are supported."
            )),
        },
        None => {
            if has_env("GEMINI_API_KEY") {
                gemini_runtime_status()
            } else if has_env("OPENAI_API_KEY") {
                openai_runtime_status()
            } else {
                AiRuntimeStatus {
                    provider: "mock".to_string(),
                    model: None,
                    reason: Some("No AI provider key is configured for this process.".to_string()),
                }
            }
        }
    }
}

#[tauri::command]
async fn generate_openai_response(instructions: String, input: String) -> Result<String, String> {
    let api_key = env_required("OPENAI_API_KEY")?;
    let model = openai_model();
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(api_key)
        .json(&json!({
            "model": model,
            "instructions": instructions,
            "input": input,
            "max_output_tokens": 900,
            "store": false
        }))
        .send()
        .await
        .map_err(|error| format!("OpenAI request failed: {error}"))?;

    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|error| format!("OpenAI response body could not be read: {error}"))?;

    if !status.is_success() {
        return Err(format!("OpenAI API returned {status}: {body}"));
    }

    extract_openai_output_text(&body)
}

#[tauri::command]
async fn generate_gemini_response(instructions: String, input: String) -> Result<String, String> {
    let api_key = env_required("GEMINI_API_KEY")?;
    let model = gemini_model();
    let client = reqwest::Client::new();
    let url =
        format!("https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent");
    let response = client
        .post(url)
        .header("x-goog-api-key", api_key)
        .json(&json!({
            "store": false,
            "systemInstruction": {
                "parts": [
                    { "text": instructions }
                ]
            },
            "contents": [
                {
                    "role": "user",
                    "parts": [
                        { "text": format!("Input JSON:
        {input}") }
                    ]
                }
            ],
            "generationConfig": {
                "temperature": 0.2,
                "maxOutputTokens": 900,
                "responseMimeType": "application/json"
            }
        }))
        .send()
        .await
        .map_err(|error| format!("Gemini request failed: {error}"))?;

    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|error| format!("Gemini response body could not be read: {error}"))?;

    if !status.is_success() {
        return Err(format!("Gemini API returned {status}: {body}"));
    }

    extract_gemini_output_text(&body)
}

fn preferred_ai_provider() -> Option<String> {
    std::env::var("AI_PROVIDER")
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
}

fn has_env(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

fn env_required(name: &str) -> Result<String, String> {
    let value =
        std::env::var(name).map_err(|_| format!("{name} is not configured for this process."))?;

    if value.trim().is_empty() {
        return Err(format!("{name} is empty."));
    }

    Ok(value)
}

fn openai_runtime_status() -> AiRuntimeStatus {
    if has_env("OPENAI_API_KEY") {
        AiRuntimeStatus {
            provider: "openai".to_string(),
            model: Some(openai_model()),
            reason: None,
        }
    } else {
        AiRuntimeStatus {
            provider: "mock".to_string(),
            model: None,
            reason: Some("OPENAI_API_KEY is not configured for this process.".to_string()),
        }
    }
}

fn gemini_runtime_status() -> AiRuntimeStatus {
    if has_env("GEMINI_API_KEY") {
        AiRuntimeStatus {
            provider: "gemini".to_string(),
            model: Some(gemini_model()),
            reason: None,
        }
    } else {
        AiRuntimeStatus {
            provider: "mock".to_string(),
            model: None,
            reason: Some("GEMINI_API_KEY is not configured for this process.".to_string()),
        }
    }
}

fn openai_model() -> String {
    std::env::var("OPENAI_MODEL")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "gpt-5-nano".to_string())
}

fn gemini_model() -> String {
    std::env::var("GEMINI_MODEL")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "gemini-3.1-flash-lite".to_string())
}

fn extract_openai_output_text(body: &str) -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(body).map_err(|error| format!("OpenAI JSON parse failed: {error}"))?;

    if let Some(text) = value.get("output_text").and_then(|text| text.as_str()) {
        return Ok(text.to_string());
    }

    let output = value
        .get("output")
        .and_then(|output| output.as_array())
        .ok_or_else(|| "OpenAI response did not include an output array.".to_string())?;

    for item in output {
        let Some(content) = item.get("content").and_then(|content| content.as_array()) else {
            continue;
        };

        for content_item in content {
            if content_item
                .get("type")
                .and_then(|item_type| item_type.as_str())
                == Some("output_text")
            {
                if let Some(text) = content_item.get("text").and_then(|text| text.as_str()) {
                    return Ok(text.to_string());
                }
            }
        }
    }

    Err("OpenAI response did not include output_text.".to_string())
}

fn extract_gemini_output_text(body: &str) -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(body).map_err(|error| format!("Gemini JSON parse failed: {error}"))?;

    for key in ["output_text", "output", "text"] {
        if let Some(text) = value.get(key).and_then(|text| text.as_str()) {
            if !text.trim().is_empty() {
                return Ok(text.to_string());
            }
        }
    }

    if let Some(steps) = value.get("steps").and_then(|steps| steps.as_array()) {
        let text = steps
            .iter()
            .filter_map(|step| step.get("content").and_then(|content| content.as_array()))
            .flat_map(|content| content.iter())
            .filter(|content_item| {
                content_item
                    .get("type")
                    .and_then(|item_type| item_type.as_str())
                    == Some("text")
            })
            .filter_map(|content_item| content_item.get("text").and_then(|text| text.as_str()))
            .collect::<Vec<_>>()
            .join("\n");

        if !text.trim().is_empty() {
            return Ok(text);
        }
    }

    if let Some(candidates) = value
        .get("candidates")
        .and_then(|candidates| candidates.as_array())
    {
        for candidate in candidates {
            if let Some(parts) = candidate
                .get("content")
                .and_then(|content| content.get("parts"))
                .and_then(|parts| parts.as_array())
            {
                let text = parts
                    .iter()
                    .filter_map(|part| part.get("text").and_then(|text| text.as_str()))
                    .collect::<Vec<_>>()
                    .join("");

                if !text.trim().is_empty() {
                    return Ok(text);
                }
            }
        }
    }

    Err("Gemini response did not include text output.".to_string())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            get_ai_runtime_status,
            generate_openai_response,
            generate_gemini_response,
            sqlite::inspect_sqlite_database,
            sqlite::initialize_sqlite_database,
            sqlite::create_sqlite_experience,
            sqlite::update_sqlite_experience,
            sqlite::delete_sqlite_experience,
            sqlite::import_sqlite_experiences,
            sqlite::execute_sqlite_transaction,
            sqlite::execute_sqlite_historical_transaction
        ])
        .run(tauri::generate_context!())
        .expect("error while running Life OS");
}

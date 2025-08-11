use base64::{Engine as _, engine::general_purpose};
use serde_json;
use chrono::{DateTime, Utc, TimeZone};

// JSON格式化
#[tauri::command]
fn format_json(input: &str) -> Result<String, String> {
    match serde_json::from_str::<serde_json::Value>(input) {
        Ok(parsed) => {
            match serde_json::to_string_pretty(&parsed) {
                Ok(formatted) => Ok(formatted),
                Err(e) => Err(format!("格式化失败: {}", e))
            }
        },
        Err(e) => Err(format!("JSON解析失败: {}", e))
    }
}

// JSON压缩
#[tauri::command]
fn minify_json(input: &str) -> Result<String, String> {
    match serde_json::from_str::<serde_json::Value>(input) {
        Ok(parsed) => {
            match serde_json::to_string(&parsed) {
                Ok(minified) => Ok(minified),
                Err(e) => Err(format!("压缩失败: {}", e))
            }
        },
        Err(e) => Err(format!("JSON解析失败: {}", e))
    }
}

// Base64编码
#[tauri::command]
fn encode_base64(input: &str) -> String {
    general_purpose::STANDARD.encode(input.as_bytes())
}

// Base64解码
#[tauri::command]
fn decode_base64(input: &str) -> Result<String, String> {
    match general_purpose::STANDARD.decode(input) {
        Ok(decoded_bytes) => {
            match String::from_utf8(decoded_bytes) {
                Ok(decoded_string) => Ok(decoded_string),
                Err(e) => Err(format!("UTF-8解码失败: {}", e))
            }
        },
        Err(e) => Err(format!("Base64解码失败: {}", e))
    }
}

// URL编码
#[tauri::command]
fn encode_url(input: &str) -> String {
    urlencoding::encode(input).to_string()
}

// URL解码
#[tauri::command]
fn decode_url(input: &str) -> Result<String, String> {
    match urlencoding::decode(input) {
        Ok(decoded) => Ok(decoded.to_string()),
        Err(e) => Err(format!("URL解码失败: {}", e))
    }
}

// 时间戳转换为日期
#[tauri::command]
fn timestamp_to_date(timestamp: i64, unit: &str) -> Result<String, String> {
    let datetime = match unit {
        "seconds" => {
            match Utc.timestamp_opt(timestamp, 0) {
                chrono::LocalResult::Single(dt) => dt,
                _ => return Err("无效的时间戳".to_string())
            }
        },
        "milliseconds" => {
            match Utc.timestamp_millis_opt(timestamp) {
                chrono::LocalResult::Single(dt) => dt,
                _ => return Err("无效的时间戳".to_string())
            }
        },
        _ => return Err("不支持的时间单位".to_string())
    };
    
    Ok(datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string())
}

// 日期转换为时间戳
#[tauri::command]
fn date_to_timestamp(date_str: &str, unit: &str) -> Result<i64, String> {
    let datetime = match DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", date_str)) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => {
            // 尝试其他格式
            match chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
                Ok(naive_dt) => Utc.from_utc_datetime(&naive_dt),
                Err(e) => return Err(format!("日期解析失败: {}", e))
            }
        }
    };
    
    match unit {
        "seconds" => Ok(datetime.timestamp()),
        "milliseconds" => Ok(datetime.timestamp_millis()),
        _ => Err("不支持的时间单位".to_string())
    }
}

// 正则表达式测试
#[tauri::command]
fn test_regex(pattern: &str, text: &str, flags: &str) -> Result<serde_json::Value, String> {
    let mut regex_builder = regex::RegexBuilder::new(pattern);
    
    if flags.contains("i") {
        regex_builder.case_insensitive(true);
    }
    if flags.contains("m") {
        regex_builder.multi_line(true);
    }
    if flags.contains("s") {
        regex_builder.dot_matches_new_line(true);
    }
    
    match regex_builder.build() {
        Ok(regex) => {
            let matches: Vec<_> = regex.find_iter(text)
                .map(|m| {
                    serde_json::json!({
                        "match": m.as_str(),
                        "start": m.start(),
                        "end": m.end()
                    })
                })
                .collect();
            
            Ok(serde_json::json!({
                "matches": matches,
                "count": matches.len(),
                "is_match": !matches.is_empty()
            }))
        },
        Err(e) => Err(format!("正则表达式错误: {}", e))
    }
}

// 生成UUID
#[tauri::command]
fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

// 哈希计算
#[tauri::command]
fn calculate_hash(input: &str, algorithm: &str) -> Result<String, String> {
    use sha2::{Sha256, Digest};
    use md5;
    
    match algorithm.to_lowercase().as_str() {
        "md5" => {
            let digest = md5::compute(input.as_bytes());
            Ok(format!("{:x}", digest))
        },
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(input.as_bytes());
            let result = hasher.finalize();
            Ok(format!("{:x}", result))
        },
        _ => Err("不支持的哈希算法".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            format_json,
            minify_json,
            encode_base64,
            decode_base64,
            encode_url,
            decode_url,
            timestamp_to_date,
            date_to_timestamp,
            test_regex,
            generate_uuid,
            calculate_hash
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

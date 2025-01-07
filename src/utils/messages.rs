use crate::utils::context::CTX_APP;
use std::{env, io};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

#[macro_export]
macro_rules! get_message {
    // array argument
    // get_message!("id.user.not.found", ["test", 2, etc...]);
    ($key: expr, $array: expr) => {{
        let mut args: Vec<&str> = Vec::new();
        for arg in $array {
            let arg = format!("{}", arg);
            args.push(arg.as_str());
        }
        let args = &args[1..args.len()];
        let message = crate::utils::messages::get_message($key, args);
        format!("{message}")
    }};

    // variadic arguments
    // get_message!("id.user.not.found", "test", 2, etc...);
    ($($args: expr),*) => {{
        let mut args: Vec<&str> = Vec::new();
        $(
            let arg = format!("{}", $args);
            args.push(arg.as_str());
        )*
        let key = args[0];
        let args = &args[1..args.len()];
        let message = crate::utils::messages::get_message(key, args);
        format!("{message}")
    }};
}

pub fn get_message(key: &str, arr: &[&str]) -> String {
    let ctx = CTX_APP.get();
    for accept_language in ctx.accept_languages {
        let prefix_key = format!("{accept_language}.{key}");
        if let Ok(mut result) = env::var(prefix_key.clone()) {
            for i in 0..arr.len() {
                let arg = format!("{{{}}}", i);
                result = result.replace(arg.as_str(), arr[i]);
            }
            return result;
        };
    }
    get_message_default("id", key, arr)
}

fn get_message_default(prefix: &str, key: &str, arr: &[&str]) -> String {
    let prefix_key = format!("{prefix}.{key}");
    match env::var(prefix_key.clone()) {
        Ok(mut result) => {
            for i in 0..arr.len() {
                let arg = format!("{{{}}}", i);
                result = result.replace(arg.as_str(), arr[i]);
            }
            result
        }
        _ => prefix_key,
    }
}

pub async fn init_message() -> io::Result<()> {
    let resources_path = env::var("RESOURCES_PATH").unwrap_or("resources".to_string());
    let mut read_dir = tokio::fs::read_dir(&resources_path).await?;
    while let Some(dir_entry) = read_dir.next_entry().await? {
        let file_type = dir_entry.file_type().await?;
        if file_type.is_file() {
            let file_name = dir_entry.file_name().to_str().unwrap().to_string();
            if file_name.starts_with("messages_") && file_name.ends_with(".properties") {
                let (first_file_name, _) = file_name.split_once(".").unwrap();
                let prefix = first_file_name.replace("messages_", "");
                let file = File::open(&format!("{}/{}", resources_path, file_name)).await?;
                let readers = BufReader::new(file);
                let mut lines = readers.lines();
                while let Some(line) = lines.next_line().await? {
                    if line.eq("") || line.starts_with("#") {
                        continue;
                    }
                    let (key, value) = line.split_once("=").unwrap();
                    env::set_var(format!("{}.{}", prefix, key.to_string()), value.to_string());
                }
            }
        }
    }

    Ok(())
}

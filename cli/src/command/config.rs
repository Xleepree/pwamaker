use serde_json::json;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn application_id_sanitize(name: &str) -> String {
    name.chars()
        .filter(|c: &char| c.is_alphanumeric())
        .collect::<String>()
}

fn binary_name_sanitize(name: &str) -> String {
    name.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "-")
        .replace("--", "-")
}

pub fn create_config(name: &str, url: &str, output_dir: impl AsRef<Path>) -> std::io::Result<()> {
    let application_id: String = format!("pwa.{}", application_id_sanitize(name));
    let binary_name: String = format!("pwa-{}", binary_name_sanitize(name));

    let config = json!({
        "$schema": "https://raw.githubusercontent.com/neutralinojs/neutralinojs/main/schemas/neutralino.config.schema.json",
        "applicationId": application_id,
        "version": "0.0.0",
        "defaultMode": "window",
        "port": 0,
        "documentRoot": "/resources/",
        "url": url,
        "enableServer": false,
        "enableNativeAPI": true,
        "tokenSecurity": "one-time",
        "logging": {
            "enabled": true,
            "writeToLogFile": true
        },
        "nativeAllowList": [
            "app.exit"
        ],
        "globalVariables": {
        },
        "modes": {
            "window": {
            "title": name,
            "width": 800,
            "height": 500,
            "minWidth": 400,
            "minHeight": 200,
            "center": true,
            "fullScreen": false,
            "alwaysOnTop": false,
            "icon": "/resources/icons/appIcon.ico",
            "enableInspector": false,
            "borderless": false,
            "maximize": false,
            "hidden": false,
            "resizable": true,
            "exitProcessOnClose": true
            }
        },
        "cli": {
            "binaryName": binary_name,
            "resourcesPath": "/resources/",
            "extensionsPath": "/extensions/",
            "clientLibrary": "/resources/js/neutralino.js",
            "binaryVersion": "6.3.0",
            "clientVersion": "6.3.0"
        }
    });

    let path: PathBuf = output_dir.as_ref().join("neutralino.config.json");
    fs::write(path, serde_json::to_string_pretty(&config).unwrap())?;
    Ok(())
}
pub mod fetch;
pub mod config;

use std::path::PathBuf;

#[derive(clap::Subcommand)]
pub enum Commands {
    Create {
        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        url: String,

        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

pub fn create_pwa(name: &str, url: &str, out: Option<PathBuf>) {
    let output_dir: PathBuf = out.unwrap_or_else(|| PathBuf::from(name));
    println!("creating PWA at {:?}", output_dir);

    // get template
    if let Err(e) = fetch::fetch_template(&output_dir) {
        eprintln!("failed to get template: {}", e);
        return;
    }

    // generate neutralino.config.json
    if let Err(e) = config::create_config(name, url, &output_dir) {
        eprintln!("failed to generate config: {}", e);
        return;
    }

    // fetch icon
    let domain: String = url
        .replace("https://", "")
        .replace("http://", "")
        .split('/')
        .next()
        .unwrap_or("favicon")
        .to_string();

    let icon_path: PathBuf = output_dir.join("resources/icons/appIcon.ico");
    let mut favicon_missing: String = String::new();
    if let Err(e) = fetch::fetch_icon(&domain, &icon_path) {
        eprintln!("failed to fetch icon: {}", e);
        favicon_missing.push_str("( FAILED TO FETCH SITE ICON, HOWEVER NON-FATAL. )");
        // not fatal: continue...
    }

    println!("created PWA. {}", favicon_missing);
}
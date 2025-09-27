use std::io::Cursor;
use std::path::Path;
use reqwest::blocking::get;
use zip::ZipArchive;
use std::fs::{self, File, create_dir_all};
use anyhow::Context;


pub fn fetch_template(output_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    // ZIP location
    let template_fetch_url: &'static str = "https://raw.githubusercontent.com/Xleepree/pwamaker/main/assets/neutralino.zip";

    // download ZIP
    let response = get(template_fetch_url)
        .context("failed to download neutralino ZIP")?
        .bytes()
        .context("failed to read ZIP response bytes")?;
    let reader= Cursor::new(response);

    // open ZIP
    let mut zip = ZipArchive::new(reader)?;

    // extract ZIP
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let outpath = output_dir.as_ref().join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

pub fn fetch_icon(domain: &str, output_path: &Path) -> anyhow::Result<()> {
    let favicon_url: String = format!("https://icons.duckduckgo.com/ip3/{}.ico", domain);
    let response = get(&favicon_url)?.bytes()?;

    if let Some(parent) = output_path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(output_path)?;
    let mut cursor  = Cursor::new(response);
    std::io::copy(&mut cursor, &mut file)?;
    Ok(())
}
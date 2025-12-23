use anyhow::{Context, Error, Result};
use colored_print::ceprintln;
use reqwest::{Client, Url};
use scraper::{ElementRef, Html};
use std::fs;
use std::io::{ErrorKind, Read, Seek};
use std::path::Path;
use zip::ZipArchive;

pub fn print_error(error: Error) {
    ceprintln!("%r:Error: %R:%b^{error}");
    for cause in error.chain().skip(1) {
        ceprintln!("%r:caused by: %R:{cause}");
    }
}

pub fn get_url(relative_url: &str) -> Result<Url> {
    const BASE_URL: &str = "https://gm48.net";
    let string = format!("{BASE_URL}/{relative_url}");
    let url = Url::parse(&string)?;
    Ok(url)
}

pub async fn get_html(client: &Client, url: Url) -> Result<Html> {
    let text: String = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let html = Html::parse_document(&text);
    Ok(html)
}

pub fn get_href(element: ElementRef) -> Result<Url> {
    let href: &str = element
        .attr("href")
        .context("Link node <a> does not contain href")?;
    let url = Url::parse(href)?;
    Ok(url)
}

/// Extract basic game (jam) metadata from a game page url:
/// "https://gm48.net/game-jams/small-world/games/habitat" returns ("small-world, "habitat")
pub fn extract_meta_from_game_url(url: &Url) -> Result<(&str, &str)> {
    let mut iter = url.path().rsplit("/").step_by(2).take(2);
    let game = iter.next().context("Could not extract game name")?;
    let jam = iter.next().context("Could not extract game jam name")?;
    Ok((jam, game))
}

pub fn mkdir(path: &Path) -> Result<()> {
    match fs::create_dir(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(e).context("Could not create directory"),
    }
}

pub fn print_archive_structure<T: Seek + Read>(archive: &mut ZipArchive<T>) {
    ceprintln!("\n%b^======== %M:ZIP Archive Structure%_: ========");
    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        let name = file.name();

        let ty: &str = match (file.is_dir(), file.is_symlink()) {
            (true, true) => "dir symlink",
            (true, false) => "dir",
            (false, true) => "file symlink",
            (false, false) => "file",
        };

        ceprintln!("[{ty}] %b:{name:?}");
    }
    eprintln!();
}

pub fn sanitize_filename(filename: &str) -> String {
    let options = sanitize_filename::Options {
        windows: true,
        truncate: true,
        replacement: "_",
    };
    sanitize_filename::sanitize_with_options(filename, options)
}

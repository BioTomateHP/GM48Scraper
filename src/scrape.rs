use crate::util::{get_href, get_html, get_url, print_archive_structure, print_error};
use anyhow::{Context, Result, bail};
use colored_print::cprintln;
use reqwest::{Client, Url};
use scraper::Selector;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use zip::ZipArchive;

pub async fn get_jams(client: &Client) -> Result<Vec<Url>> {
    let url = get_url("game-jams/top-down/games")?;
    let html = get_html(client, url).await?;

    let selector = "#jamModal .modal-body > .list-group > a";
    let selector = Selector::parse(selector).unwrap();
    let mut game_jam_links = Vec::new();

    for element in html.select(&selector) {
        let href = get_href(element)?;
        game_jam_links.push(href);
    }

    if game_jam_links.is_empty() {
        bail!("Could not find any Game Jams in HTML");
    }

    Ok(game_jam_links)
}

pub async fn get_games(client: &Client, jam_url: Url) -> Result<Vec<Url>> {
    let html = get_html(client, jam_url).await?;

    let selector = "#games .single-game > a";
    let selector = Selector::parse(selector).unwrap();
    let mut game_links = Vec::new();

    for element in html.select(&selector) {
        let href = get_href(element)?;
        game_links.push(href);
    }

    Ok(game_links)
}

pub async fn get_windows_download_url(client: &Client, game_url: Url) -> Result<Option<Url>> {
    let html = get_html(client, game_url).await?;

    let selector = "#download a.dropdown-item";
    let selector = Selector::parse(selector).unwrap();
    for element in html.select(&selector) {
        let text: String = element.text().collect();
        if text.trim() == "Windows" {
            let href = get_href(element)?;
            return Ok(Some(href));
        }
    }

    Ok(None)
}

pub async fn download_game(client: &Client, download_url: Url, file_path: &Path) -> Result<()> {
    cprintln!("Downloading game %d^{download_url}");

    let resp = client.get(download_url).send().await?;
    resp.error_for_status_ref()?;
    let bytes = resp.bytes().await?;

    let size = bytes.len();
    let human_size = humansize::format_size(size, humansize::BINARY);
    cprintln!("%B:Downloaded %u^{human_size}%_^ (%u^{size}%_^ bytes)");

    let result = tokio::task::spawn_blocking(move || extract_datafile_from_zip(&bytes)).await?;
    match result {
        Ok(data_file_content) => {
            fs::write(file_path, data_file_content)?;
        }
        Err(err) => {
            print_error(err);
        }
    }
    Ok(())
}

/// Extracts the GameMaker data file from a ZIP archive in memory.
fn extract_datafile_from_zip(data: &[u8]) -> Result<Vec<u8>> {
    let mut archive = ZipArchive::new(Cursor::new(data))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.is_dir() {
            continue;
        }

        let filename = file.name();
        let filename = filename.rsplit_once("/").map_or(filename, |x| x.1);
        if filename != "data.win" {
            continue;
        }

        let size: usize = file
            .size()
            .try_into()
            .context("File is too massive for this poor architecture")?;
        let mut content = Vec::with_capacity(size);
        std::io::copy(&mut file, &mut content)?;
        return Ok(content);
    }

    // TODO: handle SFX (self extracting exe)
    //       interface 7zip to decompress them?

    // Failed to find file, print directory for debugging and exit
    print_archive_structure(&mut archive);
    bail!("Could not find a data file in ZIP archive");
}

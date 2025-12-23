mod cli;
mod scrape;
mod util;

use crate::scrape::{download_game, get_games, get_jams, get_windows_download_url};
use crate::util::{extract_meta_from_game_url, mkdir, print_error, sanitize_filename};
use anyhow::{Context, Result};
use clap::Parser;
use colored_print::cprintln;
use reqwest::Client;
use std::process::exit;

async fn run(args: cli::Args) -> Result<()> {
    let client = &Client::new();

    mkdir(&args.directory)?;

    let jams = get_jams(client)
        .await
        .context("Could not get list of Game Jams")?;
    cprintln!("%d^Got {} game jams", jams.len());

    for game_jam_url in jams {
        cprintln!("%G:Downloading games from Game Jam {game_jam_url}");
        let games = get_games(client, game_jam_url)
            .await
            .context("Could not get list of games in Game Jam")?;
        cprintln!("%d^Got {} games", games.len());

        for game_url in games {
            let (jam, game) = extract_meta_from_game_url(&game_url)?;
            let game = urlencoding::decode(game)?;
            let filename = format!("{jam}_{game}.win");
            let filename = sanitize_filename(&filename);
            let path = args.directory.join(filename);

            if path.exists() {
                cprintln!("%y:Skipping download for {game_url}: %Y:File already exists");
                continue;
            }

            let download_url = get_windows_download_url(client, game_url.clone())
                .await
                .context("Could not get download URL for Windows")?;
            let Some(url) = download_url else {
                cprintln!(
                    "%y:Skipping download for %_:{game_url}%y:: %Y:Game does not have a Windows download"
                );
                continue;
            };

            download_game(client, url, &path).await?;
        }
    }

    cprintln!("%G:%b^All games downloaded!");
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    if let Err(e) = run(args).await {
        print_error(e);
        exit(1);
    }
}

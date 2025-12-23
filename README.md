A command line interface for scraping GameMaker data files from the [GM48 Game Jam](https://gm48.net).

## Why
The reason I made this program is to get lots of GameMaker data files (`data.win` files) in order 
to test my GameMaker asset unpacker tool [LibGM](https://github.com/BioTomateDE/LibGM).

## Usage
Currently, there is no pre-built binary available to download because I'm lazy. Please open an Issue if you would like one.

1. Install the [Rust toolchain](https://rustup.rs)
2. Clone this Git repository
3. Build the binary (`cargo build -r`)
4. Execute the binary (located in `./target/release/gm48-scraper.exe`)
 
By default, the program creates a directory called `gm48_datafiles` where it will dump all GameMaker data files.
However, you can pass a commandline argument to the CLI in order to customize your output directory.
Example: `./gm48-scraper ~/Documents/gamemaker_datafiles/`

## Is ts even allowed
I read (skimmed) their ToS and didn't find any sentence explicitly disallowing scrapers.
This program also uses the standard [reqwest](https://crates.io/crates/reqwest) user agent,
so they could explicitly block scrapers like this, if they wanted to.

That being said: **Use it on your own risk.**
I take no responsibility for lawsuits, IP-bans or any other punishment as a result of using this program.

## Contributing
All contributions are welcome!
Just open up a GitHub Issue or Pull request.

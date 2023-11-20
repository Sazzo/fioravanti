mod messages;
mod prompts;

use std::fs::File;

use clap::Parser;
use messages::*;
use prompts::*;
use rodio::{Decoder, OutputStream, Sink};
use serde_json::Map;

#[derive(Parser, Debug)]
struct CliArgs {
    // Number of the sound effect to play
    #[arg(short, long, required = false)]
    sfx: Option<u32>,
}

fn main() {
    println!("{}", FIORAVANTI_ASCII_TEXT);
    println!("{}", WELCOME_MESSAGE);

    let args = CliArgs::parse();

    let sfx_list_file =
        std::fs::read_to_string("sounds.json").expect("Oops, sounds.json not found!");
    let sfx_map: Map<String, serde_json::Value> = serde_json::from_str(&sfx_list_file).unwrap();
    if args.sfx.is_none() {
        let sfx_list: Vec<String> = sfx_map.keys().map(|s| s.to_string()).collect();

        loop {
            let selected_sfx = prompt_select_sfx(&sfx_list);
            let selected_sfx_file = &sfx_map[&sfx_list[selected_sfx]];

            /* let progress_bar = ProgressBar::new(300);
            progress_bar.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} {msg}").unwrap()
                    .progress_chars("##-"),
            );
            progress_bar.set_message(format!("Tocando {}", selected_sfx_file)); */

            // TODO: Add SFX remaining duration.
            println!(
                "...Tocando {0} do SFX {1}. Aguarde terminar ou use CTRL+C para terminar.",
                selected_sfx_file, selected_sfx
            );

            play_sfx(&selected_sfx_file.as_str().unwrap());

            // progress_bar.finish();
        }
    }
}

fn play_sfx(sfx_file: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let loaded_sfx_file =
        File::open(format!("sounds/{}", sfx_file)).expect("Oops, sounds file not found!");
    let sfx_source = Decoder::new(loaded_sfx_file).unwrap();

    sink.append(sfx_source);

    sink.sleep_until_end();
}

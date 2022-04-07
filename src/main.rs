#[macro_use]
mod macros;
pub mod io;
mod keymap;
mod templates;
mod types;

//use macros::*;
mod combos;

use clap::{Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Debug, Parser)]
#[clap(name = "kbtools")]
#[clap(about = "Generate C codes for my QMK", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    GenKeyMap {
        keymap: std::path::PathBuf,
    },
    GenCombos {
        keymap: std::path::PathBuf,
        combos: std::path::PathBuf,
    },
    GenComboTerms {
        combos: std::path::PathBuf,
    }
}
use askama::Template; // bring trait in scope
fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::GenKeyMap { keymap } => {
            let keymap = io::load_key_map_json(keymap); //"../keymaps/jerris.json");
            let template = templates::KeyMapCTemplate {
                layers: &keymap.layers,
            };
            let res = template.render().unwrap();
            println!("{}", &res);
        }
        Commands::GenComboTerms { combos } => {
            let combos = io::load_combos_json(combos); // "../keymaps/combos.dhall");
            combos::generate_combo_terms(&combos);
        },
        Commands::GenCombos { keymap, combos } => {
            let keymap = io::load_key_map_json(keymap); //"../keymaps/jerris.json");
            let combos = io::load_combos_json(combos); // "../keymaps/combos.dhall");
            let generated_combos = combos::resolve_combo_keys(&combos, &keymap);
            println!("{}", generated_combos.join("\n"));            
        }
    }
}

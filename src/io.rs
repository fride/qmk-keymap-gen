use crate::types::{Combo, ComboBinding, Combos, Keymap};
use serde::{Deserialize, Serialize};

use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct KeymapJson {
    pub keyboard: String,
    pub keymap: String,
    pub layout: String,
    pub layers: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ComboJson {
    Comb {
        key_positions: Vec<usize>,
        bindings: String,
        timeout_ms: usize,
    },
    Subs {
        key_positions: Vec<usize>,
        send_string: String,
        timeout_ms: usize,
    },
}

pub fn load_key_map_json<P: AsRef<Path>>(path: P) -> Keymap {
    let contents = std::fs::read_to_string(path).expect("Could not open file");
    let keymap_json: KeymapJson = serde_dhall::from_str(&contents).parse().expect("Could not parse json");
    Keymap {
        keyboard: keymap_json.keyboard,
        keymap: keymap_json.keymap,
        layout: keymap_json.layout,
        layers: keymap_json.layers.into(),
    }
}

pub fn load_combos_json<P: AsRef<Path>>(path: P) -> Combos {
    let contents = std::fs::read_to_string(path).expect("Could not open file");
    let combo_json: std::collections::BTreeMap<String, ComboJson> =
        serde_dhall::from_str(&contents).parse().unwrap(); //..expect("Could not parse json");

    let mut combos = vec![];
    for (name, combo) in combo_json {
        let combo = match combo {
            ComboJson::Comb {
                key_positions,
                timeout_ms,
                bindings,
            } => Combo {
                name: name.to_owned(),
                bindings: ComboBinding::KeyPress(bindings),
                timeout_ms,
                key_positions
            },
            ComboJson::Subs {
                key_positions,
                timeout_ms,
                send_string,
            } => Combo {
                name: name.to_owned(),
                bindings: ComboBinding::Subs(send_string),
                timeout_ms,
                key_positions
            },
        };
        combos.push(combo);
    }
    combos.into()
}

use serde::{Deserialize, Serialize};
use std::{fmt, ops::Index};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyCode(String);
impl KeyCode {
    pub fn new(s: &str) -> Self {
        use regex::Regex;
        let any_r = Regex::new(r"^ANY\((.+)\)$").unwrap();
        let any_key = any_r.captures_iter(s).next();
        match any_key {
            Some(a) => KeyCode(a[1].to_owned()),
            _ => KeyCode(s.to_owned()),
        }
    }
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl std::convert::From<&str> for KeyCode {
    fn from(s: &str) -> Self {
        KeyCode::new(s)
    }
}

impl std::fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// TODO: Do I need this!?
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardLayout(Vec<usize>);
impl KeyboardLayout {
    pub fn from_name(n: &str) -> Option<Self> {
        if n == "LAYOUT_split_3x5_2" {
            Some(KeyboardLayout(vec![10, 10, 10, 2]))
        } else {
            None
        }
    }
    pub fn key_count(&self) -> usize {
        self.0.iter().sum::<usize>().clone()
    }

    pub fn row_size(&self, row: usize) -> usize {
        if self.0.len() <= row {
            self.0[row]
        } else {
            0
        }
    }
    pub fn index(&self, row: usize, col: usize) -> usize {
        let row_offset = &self.0.iter().take(row).sum::<usize>();
        row_offset + col
    }

    pub fn row_for_index(&self, index: usize) -> usize {
        let mut row = 0;
        let mut num_keys = 0;
        for row_len in &self.0 {
            num_keys = num_keys + row_len;
            if index < num_keys {
                return row;
            }
            row = row + 1
        }
        panic!("index {} is out of bound", index);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layer {
    pub index: usize,
    pub key_codes: Vec<KeyCode>,
}
impl Index<usize> for Layer {
    type Output = KeyCode;
    fn index(&self, index: usize) -> &Self::Output {
        &self.key_codes[index]
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layers(Vec<Layer>);
impl Layers {
    pub fn iter(&self) -> std::slice::Iter<Layer> {
        self.0.iter()
    }
}

impl Index<usize> for Layers {
    type Output = Layer;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IntoIterator for Layers {
    type Item = Layer;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::convert::From<Vec<Vec<String>>> for Layers {
    fn from(source: Vec<Vec<String>>) -> Self {
        fn go(mut acc: Vec<Layer>, row: Vec<String>) -> Vec<Layer> {
            let index = acc.len();
            let key_codes = row.into_iter().map(|k| KeyCode::new(&k)).collect();
            let layer = Layer { index, key_codes };
            acc.push(layer);
            acc
        }
        let layers = source.into_iter().fold(vec![], go);
        Self(layers)
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Keymap {
    pub keyboard: String,
    pub keymap: String,
    pub layout: String,
    pub layers: Layers,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ComboBinding {
    KeyPress(String),
    Subs(String),
}
impl ComboBinding {
    pub fn to_string(&self) -> String {
        match self {
            ComboBinding::KeyPress(s) => s.clone(),
            ComboBinding::Subs(s) => s.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Combo {
    pub name: String,
    pub key_positions: Vec<usize>,
    pub bindings: ComboBinding,
    pub timeout_ms: usize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Combos(Vec<Combo>);
impl Combos {
    pub fn iter(&self) -> std::slice::Iter<Combo> {
        self.0.iter()
    }
}
impl std::convert::From<Vec<Combo>> for Combos {
    fn from(source: Vec<Combo>) -> Self {
        Self(source)
    }
}

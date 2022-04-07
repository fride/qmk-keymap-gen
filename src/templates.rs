use std::borrow::Borrow;

use askama::Template; // bring trait in scope

use crate::types::*;

// needed to join keycodes
impl Borrow<str> for KeyCode {
    fn borrow(&self) -> &str {
        self.name()
    }
}
#[derive(Template)]
#[template(path = "keymap.txt", ext = "txt")]
pub struct KeyMapCTemplate<'a> {
    pub layers: &'a Layers,
}

#[derive(Template)]
#[template(path = "combos.def", ext = "txt", escape = "none")]
pub struct CombosDefTemplate<'a> {
    pub combos: &'a Combos,
}

// use super::Combo;
// use super::Keyboard;
// use super::Layer;

// #[derive(Template)] // this will generate the code...
// #[template(path = "combos.txt")] // using the template in this path, relative
//                                  // to the `templates` dir in the crate root
// pub struct HelloTemplate {
//     // the name of the struct can be anything
//     combos: Vec<String>, // the field name should match the variable name
//                          // in your template
// }

// fn key_indexes(combo: &Combo, keyboard: &Keyboard) -> String {
//     let mut indexes: Vec<String> = vec![];
//     for idx in combo.key_indexes.iter() {
//         let key = keyboard.key_at(idx.clone(), 0).unwrap();
//         let s = format!("{key}", key = key);
//         println!("key: {}", &s);
//         indexes.push(s);
//     }
//     indexes.join(",")
// }
// pub fn combos(keyboard: &Keyboard) -> String {
//     let mut combos = vec![];
//     for combo in keyboard.combos.iter() {
//         combos.push(format!(
//             "COMB({},       {},     {})",
//             combo.name,
//             format!("{}", combo.behavior),
//             key_indexes(&combo, &keyboard)
//         ));
//     }

//     let combostemplate = HelloTemplate { combos };
//     format!("{}", combostemplate.render().unwrap())
// }

// // #[derive(Template)] // this will generate the code...
// // #[template(path = "layers.txt")] // using the template in this path, relative
// //                                  // to the `templates` dir in the crate root
// // pub struct LayersTemplate { // the name of the struct can be anything
// //     pub layers: Vec<Vec<String>>
// //                    // in your template
// // }

// // impl LayersTemplate {
// //     pub fn new(kb: &Keyboard) -> Self {
// //         let mut layers = vec![];
// //         // for layer in &kb.layers {
// //         //    let mut layer = vec![];
// //         //    for row in &layer.key_bindings {

// //         //    }
// //         // }
// //         Self { layers }
// //     }
// // }

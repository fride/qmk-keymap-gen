use crate::types::*;
use askama::Template; // bring trait in scope

pub fn resolve_combo_keys(combos: &Combos, keymap: &Keymap) -> Vec<String>{
    let layer = &keymap.layers[0];

    let mut generated_combos = vec![];
    for combo in combos.iter() {
        let key_codes = combo
            .key_positions
            .iter()
            .map(|idx| format!("{}", &layer[idx.clone()]))
            .collect::<Vec<String>>()
            .join(",");
        let combo_str = match &combo.bindings {
            ComboBinding::KeyPress(key) => {
                format!("COMB({},\t\t{},\t\t{})", &combo.name, &key, &key_codes)
            }
            ComboBinding::Subs(send_stromg) => format!(
                "SUBS({},\t\t\"{}\",\t\t{})",
                &combo.name, &send_stromg, &key_codes
            ),
        };
        generated_combos.push(combo_str);        
    }
    generated_combos
}

#[derive(Template)]
#[template(path = "combo_terms.c", ext = "txt", escape = "none")]struct ComboTermsTemplate<'a> {
    pub combos: & 'a Combos
}
pub fn generate_combo_terms(combos: &Combos) {
    let template = ComboTermsTemplate { combos };
    println!("{}", template.render().unwrap());
}